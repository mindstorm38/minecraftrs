use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::collections::HashMap;
use std::ptr::NonNull;

use super::{Block, UntypedProperty, Property, PropertySerializable};
use crate::util::OpaquePtr;


/// The maximum number of states for a single block.
pub const MAX_STATES_COUNT: usize = 0x10000;


#[derive(Debug)]
pub(crate) struct SharedProperty {
    prop: &'static dyn UntypedProperty,
    index: usize,
    length: u8,
    period: usize
}


/// Represent a particular state of a block, this block state also know
/// all its neighbors by their properties and values.
///
/// To build states, use `BlockStateContainerBuilder` and add all wanted
/// properties.
pub struct BlockState {
    /// The index of this state within the shared data's states vector.
    index: u16,
    /// Array of property encoded values.
    properties: Vec<u8>,
    /// Circular reference back to the owner
    block: NonNull<Block>
}

pub type BlockStateKey = OpaquePtr<BlockState>;

unsafe impl Send for BlockState {}
unsafe impl Sync for BlockState {}


impl BlockState {

    pub(crate) fn build_singleton() -> BlockState {
        BlockState {
            index: 0,
            properties: Vec::new(),
            block: NonNull::dangling()
        }
    }

    pub(crate) fn build_complex(properties: &[&'static dyn UntypedProperty]) -> (HashMap<&'static str, SharedProperty>, Vec<BlockState>) {

        debug_assert!(!properties.is_empty(), "building complex states without properties is not allowed");

        let mut states_count = 1;
        let mut properties_periods = Vec::with_capacity(properties.len());

        for &prop in properties {
            let length = prop.len();
            states_count *= length as usize;
            properties_periods.push((prop, length, 1usize));
        }

        if states_count > MAX_STATES_COUNT {
            panic!("Too many properties for this state, the maximum number is {}.", MAX_STATES_COUNT);
        }

        let mut shared_properties = HashMap::with_capacity(properties.len());

        let mut next_period = 1;
        for (i, (prop, length, period)) in properties_periods.iter_mut().enumerate().rev() {
            let prop = *prop;
            *period = next_period;
            next_period *= *length as usize;
            shared_properties.insert(prop.name(), SharedProperty {
                prop,
                index: i,
                length: *length,
                period: *period
            });
        }

        let mut shared_states = Vec::with_capacity(states_count);

        for i in 0..states_count {

            let mut state_properties = Vec::with_capacity(properties.len());
            for (_, length, period) in &properties_periods {
                state_properties.push(((i / *period) % (*length as usize)) as u8);
            }

            shared_states.push(BlockState {
                index: i as u16,
                properties: state_properties,
                block: NonNull::dangling()
            });

        }

        (shared_properties, shared_states)

    }

    #[inline]
    pub fn get_index(&self) -> u16 {
        self.index
    }

    #[inline]
    pub fn get_key(&'static self) -> BlockStateKey {
        OpaquePtr::new(self)
    }

    #[inline]
    pub fn get_block(&self) -> &'static Block {
        // SAFETY: This pointer is always valid since:
        //  - block state must be owned by a Block, and this Block must be pined in a box
        //  - this function is not called before the pointer initialization (before set_block)
        unsafe { self.block.as_ref() }
    }

    /// Really unsafe method, should only be called by `Block` constructor.
    #[inline]
    pub(super) fn set_block(&mut self, block: NonNull<Block>) {
        // This method is called once in Block::new
        self.block = block;
    }

    #[inline]
    pub fn is_block(&self, block: &'static Block) -> bool {
        self.get_block() == block
    }

    #[inline]
    fn get_block_shared_prop(&self, name: &str) -> Option<&SharedProperty> {
        self.get_block().get_storage().get_shared_prop(name)
    }

    /// Get a block state property value if the property exists.
    pub fn get<T, P>(&self, property: &P) -> Option<T>
    where
        T: PropertySerializable,
        P: Property<T>
    {

        let prop = self.get_block_shared_prop(&property.name())?;
        if prop.prop.type_id() == property.type_id() {
            property.decode(self.properties[prop.index])
        } else {
            None
        }

    }

    pub fn expect<T, P>(&self, property: &P) -> T
    where
        T: PropertySerializable,
        P: Property<T>
    {
        self.get(property).unwrap()
    }

    /// Try to get a neighbor with all the same properties excepts the given one with the given
    /// value, if the property or its value is not valid for the block, None is returned.
    pub fn with<T, P>(&self, property: &P, value: T) -> Option<&BlockState>
    where
        T: PropertySerializable,
        P: Property<T>
    {
        let prop = self.get_block_shared_prop(&property.name())?;
        self.with_unchecked(prop, property.encode(value)?)
    }

    /// Try to get a neighbor with all the same properties excepts the given one with the given
    /// value, if the property or its value is not valid for the block, None is returned.
    ///
    /// This version of `with` method take raw property name and value as strings.
    pub fn with_raw(&self, prop_name: &str, prop_value: &str) -> Option<&BlockState> {
        let prop = self.get_block_shared_prop(prop_name)?;
        self.with_unchecked(prop, prop.prop.prop_from_string(prop_value)?)
    }

    #[inline]
    fn with_unchecked(&self, prop: &SharedProperty, prop_value: u8) -> Option<&BlockState> {

        let new_value = prop_value as isize;
        let current_value = self.properties[prop.index] as isize;

        Some(if new_value == current_value {
            self
        } else {
            let value_diff = new_value - current_value;
            let neighbor_index = (self.index as isize + value_diff * prop.period as isize) as usize;
            self.get_block().get_storage().get_state_unchecked(neighbor_index)
        })

    }

    /// Iterate over representations of each property of this state.
    /// No iterator is returned if the underlying block as no other state other than this one.
    pub fn iter_raw_states<'a>(&'a self) -> Option<impl Iterator<Item = (&'static str, String)> + 'a> {
        self.get_block().get_storage().get_shared_props().map(move |props| {
            props.iter().map(move |(&name, shared)| {
                let raw_value = self.properties[shared.index];
                (name, shared.prop.prop_to_string(raw_value).unwrap())
            })
        })
    }

}


// Custom implementation for static block state references.
impl PartialEq for &'static BlockState {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}

impl Eq for &'static BlockState {}


impl Debug for BlockState {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        let reprs = match self.iter_raw_states() {
            Some(it) => it.collect(),
            None => Vec::with_capacity(0)
        };

        f.debug_struct("BlockState")
            .field("block", &self.get_block().get_name())
            .field("index", &self.index)
            .field("properties", &reprs)
            .field("raw_properties", &self.properties)
            .finish()

    }

}