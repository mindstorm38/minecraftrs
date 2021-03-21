use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::collections::{HashMap, HashSet};
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
    /// Unique ID of this state within its block.
    uid: u16,
    /// The index of this state within the shared data's states vector.
    index: usize,
    /// Array of property encoded values.
    properties: Vec<u8>,
    /// Circular reference back to the owner
    block: NonNull<Block>
}


/// A builder used to build all block states according to a list of properties.
///
/// First register your properties using the `prop` then build the states vec
/// using `build`.
pub struct BlockStateBuilder(Option<BlockStateBuilderData>);

/// Lazy internal data for BlockStateBuilder.
struct BlockStateBuilderData {
    properties_names: HashSet<&'static str>,
    properties: Vec<&'static dyn UntypedProperty>
}


impl BlockStateBuilder {

    pub fn new() -> Self {
        BlockStateBuilder(None)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        BlockStateBuilder(if capacity == 0 {
            None
        } else {
            Some(BlockStateBuilderData {
                properties_names: HashSet::with_capacity(capacity),
                properties: Vec::with_capacity(capacity)
            })
        })
    }

    fn get_data(&mut self) -> &mut BlockStateBuilderData {
        self.0.get_or_insert_with(|| BlockStateBuilderData {
            properties_names: HashSet::new(),
            properties: Vec::new()
        })
    }

    /// Register a property to add to the built states. Properties are indexed
    /// by their name, so this method will panic if you add properties with the
    /// same name.
    pub fn prop(mut self, property: &'static impl UntypedProperty) -> Self {
        let data = self.get_data();
        if data.properties_names.contains(&property.name()) {
            panic!("Property '{}' already registered.", property.name())
        } else {
            if property.len() == 0 {
                panic!("Property '{}' length is 0.", property.name());
            } else {
                data.properties_names.insert(property.name());
                data.properties.push(property);
                self
            }
        }
    }

    /// Build and resolve all combinations of property values as block states, all resolved
    /// block states know their neighbors by property and values.
    ///
    /// The method will panic if the UID overflow the
    pub(super) fn build(self) -> (HashMap<&'static str, SharedProperty>, Vec<BlockState>) {

        let mut states_count = 1;

        let (
            properties_periods,
            shared_properties
        ) = if let Some(data) = self.0 {

            let mut properties_periods = Vec::with_capacity(data.properties.len());

            for &prop in &data.properties {
                let length = prop.len();
                states_count *= length as usize;
                properties_periods.push((prop, length, 1usize));
            }

            if states_count > MAX_STATES_COUNT {
                panic!("Too many properties for this state, the maximum number is {}.", MAX_STATES_COUNT);
            }

            let mut shared_properties = HashMap::with_capacity(data.properties.len());

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

            (Some(properties_periods), shared_properties)

        } else {
            (None, HashMap::with_capacity(0))
        };

        let mut shared_states = Vec::with_capacity(states_count);

        for i in 0..states_count {

            let state_properties = if let Some(periods) = &properties_periods {
                let mut props = Vec::with_capacity(periods.len());
                for (_, length, period) in periods.iter() {
                    props.push(((i / *period) % (*length as usize)) as u8);
                }
                props
            } else {
                Vec::with_capacity(0)
            };

            shared_states.push(BlockState {
                uid: i as u16,
                index: i,
                properties: state_properties,
                block: NonNull::dangling()
            });

        }

        (shared_properties, shared_states)

    }

}


impl BlockState {

    pub fn get_uid(&self) -> u16 {
        self.uid
    }

    pub fn get_block(&self) -> &Block {
        // SAFETY: This pointer is always valid since:
        //  - block state must be owned by a Block, and this Block must be pined in a box
        //  - this function is not called before the pointer initialization (before set_block)
        unsafe { self.block.as_ref() }
    }

    pub(super) fn set_block(&mut self, block: NonNull<Block>) {
        // This method is called once in the Block constructor.
        self.block = block;
    }

    /// Get a block state property value if the property exists.
    pub fn get<T, P>(&self, property: &P) -> Option<T>
        where
            T: PropertySerializable,
            P: Property<T>
    {

        let prop = self.get_block().properties.get(&property.name())?;

        if prop.prop.type_id()  == property.type_id() {
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
        let prop = block.properties.get(&property.name())?;

        let new_value = property.encode(value)? as isize;
        let current_value = self.properties[prop.index] as isize;

        Some(if new_value == current_value {
            &block.states[self.index]
        } else {
            let value_diff = new_value - current_value;
            let neighbor_index = (self.index as isize + value_diff * prop.period as isize) as usize;
            &block.states[neighbor_index]
        })

    }

}


impl Debug for BlockState {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        let mut properties = HashMap::new();

        for shared_prop in self.get_block().properties.values() {
            let prop = shared_prop.prop;
            properties.insert(
                prop.name(),
                prop.prop_to_string(self.properties[shared_prop.index]).unwrap()
            );
        }

        f.debug_struct("BlockState")
            .field("block", &self.get_block().get_name())
            .field("uid", &self.uid)
            .field("index", &self.index)
            .field("properties", &properties)
            .field("raw_properties", &self.properties)
            .finish()

    }

}