use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::collections::{HashMap, HashSet};
use std::sync::{Weak, Arc};
use std::any::TypeId;

use super::{BlockSharedData, UntypedProperty, Property, PropertySerializable};
use crate::util;


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
    /// Unique ID of this state within its register.
    pub(crate) uid: u16,
    /// The index of this state within the shared data's states vector.
    index: usize,
    /// Array of property encoded values.
    properties: Vec<u8>,
    /// Shared data among all block states.
    pub(crate) shared_data: Weak<BlockSharedData>,
}


/// A builder used to build all block states according to a list of properties.
///
/// First register your properties using the `prop` then build the states vec
/// using `build`.
pub struct BlockStateBuilder {
    properties_names: HashSet<&'static str>,
    properties: Vec<&'static dyn UntypedProperty>
}


impl BlockStateBuilder {

    pub fn new() -> Self {
        BlockStateBuilder {
            properties_names: HashSet::new(),
            properties: Vec::new()
        }
    }

    /// Register a property to add to the built states. Properties are indexed
    /// by their name, so this method will panic if you add properties with the
    /// same name.
    pub fn prop(mut self, property: &'static impl UntypedProperty) -> Self {

        if self.properties_names.contains(&property.name()) {
            panic!("Property '{}' already registered.", property.name())
        } else {
            if property.len() == 0 {
                panic!("Property '{}' length is 0.", property.name());
            } else {
                self.properties_names.insert(property.name());
                self.properties.push(property);
                self
            }
        }

    }

    /// Build and resolve all combinations of property values as block states, all resolved
    /// block states know their neighbors by property and values.
    ///
    /// The given `reg_tid` (Register Type ID) must be the type id of the structure that will hold
    /// the block for these states. The passed UID is the next uid to set and increment for
    /// each state (pass 0 for the first state builder, then the first state will have uid 0).
    ///
    /// The method will panic if the UID overflow the max for `u16`.
    pub(crate) fn build(self,
                        reg_tid: TypeId,
                        uid: &mut u16,
                        states_by_uid: &mut HashMap<u16, Weak<BlockState>>
    ) -> Arc<BlockSharedData> {

        let mut states_count = 1;
        let mut properties_periods = Vec::with_capacity(self.properties.len());

        for &prop in &self.properties {
            let length = prop.len();
            states_count *= length as usize;
            properties_periods.push((prop, length, 1usize));
        }

        if *uid as usize + states_count > u16::MAX as usize {
            panic!("Not enough states UIDs to store these block states.");
        }

        // We know that the states count is valid, so we allocate the block's shared data.
        let shared_data_arc = Arc::new(
            BlockSharedData::new(reg_tid, self.properties.len(), states_count)
        );

        // This doesn't work because the lifetime of this mutable borrow last for
        //  the whole states loops, but we need to make immutable borrows in the loop:
        // => let shared_data = Arc::get_mut(&mut shared_data_arc).unwrap();

        // SAFETY: Shared data can be mutated because at this point the data is not
        //   changed by shared cloned Arc in this method.
        let shared_data = unsafe { util::mutate_ref(&*shared_data_arc) };
        let shared_properties = &mut shared_data.properties;
        let shared_states = &mut shared_data.states;

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

        for i in 0..states_count {

            let mut state_properties = Vec::with_capacity(properties_periods.len());

            for (_, length, period) in properties_periods.iter() {
                state_properties.push(((i / *period) % (*length as usize)) as u8);
            }

            let state = Arc::new(BlockState {
                uid: *uid,
                index: i,
                properties: state_properties,
                shared_data: Arc::downgrade(&shared_data_arc)
            });

            states_by_uid.insert(state.uid, Arc::downgrade(&state));
            shared_states.push(state);

            *uid += 1;

        }

        shared_data.default_state = Arc::downgrade(&shared_states[0]);

        shared_data_arc

    }

}


impl BlockState {

    pub fn get_uid(&self) -> u16 {
        self.uid
    }

    /// Get a block state property value if the property exists.
    pub fn get<T, P>(&self, property: &P) -> Option<T>
        where
            T: PropertySerializable,
            P: Property<T>
    {

        let data = self.shared_data.upgrade().unwrap();
        let prop = data.properties.get(&property.name())?;

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
    pub fn with<T, P>(&self, property: &P, value: T) -> Option<Arc<BlockState>>
        where
            T: PropertySerializable,
            P: Property<T>
    {

        // SAFETY: Shared data is weak, but we expect it to exist.
        let data = self.shared_data.upgrade().unwrap();
        let prop = data.properties.get(&property.name())?;

        let new_value = property.encode(value)? as isize;
        let current_value = self.properties[prop.index] as isize;

        let state = if new_value == current_value {
            &data.states[self.index]
        } else {
            let value_diff = new_value - current_value;
            let neighbor_index = (self.index as isize + value_diff * prop.period as isize) as usize;
            &data.states[neighbor_index]
        };

        Some(Arc::clone(state))

    }

}


impl Debug for BlockState {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        let mut properties = HashMap::new();

        let shared_data = self.shared_data.upgrade().unwrap();
        for shared_prop in shared_data.properties.values() {
            let prop = shared_prop.prop;
            properties.insert(
                prop.name(),
                prop.prop_to_string(self.properties[shared_prop.index]).unwrap()
            );
        }

        f.debug_struct("BlockState")
            .field("uid", &self.uid)
            .field("index", &self.index)
            .field("properties", &properties)
            .field("raw_properties", &self.properties)
            .finish()

    }

}