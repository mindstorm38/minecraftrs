use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::collections::HashMap;
use std::ptr::NonNull;

use super::{Block, UntypedProperty, Property, PropertySerializable};


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

    pub fn get_index(&self) -> u16 {
        self.index
    }

    pub fn get_block(&self) -> &Block {
        // SAFETY: This pointer is always valid since:
        //  - block state must be owned by a Block, and this Block must be pined in a box
        //  - this function is not called before the pointer initialization (before set_block)
        unsafe { self.block.as_ref() }
    }

    #[inline]
    pub(super) fn set_block(&mut self, block: NonNull<Block>) {
        // This method is called once in Block::new
        self.block = block;
    }

    /// Get a block state property value if the property exists.
    pub fn get<T, P>(&self, property: &P) -> Option<T>
        where
            T: PropertySerializable,
            P: Property<T>
    {

        let prop = self.get_block().get_shared_prop(&property.name())?;
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

    /// Try to get this a neighbor with all the same properties excepts the given one with the given
    /// value, if the property or its value is not valid for the block, None is returned.
    pub fn with<T, P>(&self, property: &P, value: T) -> Option<&BlockState>
        where
            T: PropertySerializable,
            P: Property<T>
    {

        let block = self.get_block();
        let prop = block.get_shared_prop(&property.name())?;

        let new_value = property.encode(value)? as isize;
        let current_value = self.properties[prop.index] as isize;

        Some(if new_value == current_value {
            self
        } else {
            let value_diff = new_value - current_value;
            let neighbor_index = (self.index as isize + value_diff * prop.period as isize) as usize;
            block.get_state_unchecked(neighbor_index)
        })

    }

}


impl Debug for BlockState {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        let reprs = match self.get_block().get_shared_props() {
            None => HashMap::new(),
            Some(shared_properties) => {
                let mut reprs = HashMap::new();
                for shared_prop in shared_properties.values() {
                    let prop = shared_prop.prop;
                    reprs.insert(
                        prop.name(),
                        prop.prop_to_string(self.properties[shared_prop.index]).unwrap()
                    );
                }
                reprs
            }
        };

        f.debug_struct("BlockState")
            .field("block", &self.get_block().get_name())
            .field("index", &self.index)
            .field("properties", &reprs)
            .field("raw_properties", &self.properties)
            .finish()

    }

}