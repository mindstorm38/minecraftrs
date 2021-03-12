use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::{Weak, Arc};
use std::str::FromStr;
use std::time::Instant;

use super::{Block, BlockSharedData};
use crate::util;


/// All valid property values types must implement this trait.
pub trait PropertySerializable: 'static + Copy {
    fn prop_to_string(self) -> String;
    fn prop_from_string(value: &str) -> Option<Self>;
}


/// Trait for all properties stored in a block state.
pub trait Property<T: PropertySerializable>: Any {
    fn name(&self) -> &'static str;
    fn len(&self) -> u8;
    fn encode(&self, value: T) -> Option<u8>;
    fn decode(&self, index: u8) -> Option<T>;
}


impl<T> PropertySerializable for T
where
    T: 'static + Copy + ToString + FromStr
{

    fn prop_to_string(self) -> String {
        self.to_string()
    }

    fn prop_from_string(value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }

}


pub struct BoolProperty(pub &'static str);

impl Property<bool> for BoolProperty {

    fn name(&self) -> &'static str {
        self.0
    }

    fn len(&self) -> u8 {
        2
    }

    fn encode(&self, value: bool) -> Option<u8> {
        Some(if value { 1 } else { 0 })
    }

    fn decode(&self, index: u8) -> Option<bool> {
        Some(index != 0)
    }

}


pub struct IntProperty(pub &'static str, pub u8);

impl Property<u8> for IntProperty {

    fn name(&self) -> &'static str {
        self.0
    }

    fn len(&self) -> u8 {
        self.1
    }

    fn encode(&self, value: u8) -> Option<u8> {
        if value < self.1 { Some(value) } else { None }
    }

    fn decode(&self, index: u8) -> Option<u8> {
        if index < self.1 { Some(index) } else { None }
    }
}


pub struct EnumProperty<T: PropertySerializable + Eq>(pub &'static str, pub &'static [T]);

impl<T> Property<T> for EnumProperty<T>
where
    T: PropertySerializable + Eq
{

    fn name(&self) -> &'static str {
        self.0
    }

    fn len(&self) -> u8 {
        self.1.len() as u8
    }

    fn encode(&self, value: T) -> Option<u8> {
        Some(self.1.iter().position(|v| *v == value)? as u8)
    }

    fn decode(&self, index: u8) -> Option<T> {
        Some(*(self.1.get(index as usize)?))
    }

}


#[macro_export]
macro_rules! properties {
    ($v:vis $id:ident: int($name:literal, $count:expr); $($t:tt)*) => {
        $v static $id: $crate::block::IntProperty = $crate::block::IntProperty($name, $count);
        properties!($($t)*);
    };
    ($v:vis $id:ident: bool($name:literal); $($t:tt)*) => {
        $v static $id: $crate::block::BoolProperty = $crate::block::BoolProperty($name);
        properties!($($t)*);
    };
    ($v:vis $id:ident: enum<$enum_type:ty>($name:literal, $values_id:ident, [$($value:ident),+]); $($t:tt)*) => {
        static $values_id: [$enum_type; 0 $(+ (1, <$enum_type>::$value).0)+] = [$(<$enum_type>::$value),+];
        $v static $id: $crate::block::EnumProperty<$enum_type> = $crate::block::EnumProperty($name, &$values_id);
        properties!($($t)*);
    };
    ($v:vis $id:ident: enum<$enum_type:ty>($name:literal, $values_id:ident); $($t:tt)*) => {
        $v static $id: $crate::block::EnumProperty<$enum_type> = $crate::block::EnumProperty($name, &$values_id);
        properties!($($t)*);
    };
    () => {}
}


#[macro_export]
macro_rules! impl_enum_serializable {
    ($enum_id:ident { $($item_id:ident: $item_name:literal),* }) => {
        impl $crate::block::PropertySerializable for $enum_id {
            fn prop_to_string(self) -> String {
                match self {
                    $(Self::$item_id => $item_name),*
                }.to_string()
            }
            fn prop_from_string(value: &str) -> Option<Self> {
                match value {
                    $($item_name => Some(Self::$item_id),)*
                    _ => None
                }
            }
        }
    };
}


#[derive(Debug)]
pub(crate) struct SharedProperty {
    type_id: TypeId,
    index: usize,
    length: u8,
    period: usize
}


/// Represent a particular state of a block, this block state also know
/// all its neighbors by their properties and values.
///
/// To build states, use `BlockStateContainerBuilder` and add all wanted
/// properties.
#[derive(Debug)]
pub struct BlockState {
    /// Unique ID of this state within its register.
    pub(crate) uid: u16,
    /// Unique TypeId of the register this state belongs to.
    pub(crate) reg_tid: TypeId,
    /// The index of this state within the shared data's states vector.
    index: usize,
    /// Array of property encoded values.
    properties: Vec<u8>,
    ///
    shared_data: Weak<BlockSharedData>,
}


/// A builder used to build all block states according to a list of properties.
///
/// First register your properties using the `prop` then build the states vec
/// using `build`.
pub struct BlockStateBuilder {
    properties: HashMap<&'static str, (TypeId, u8)>,
}


impl BlockStateBuilder {

    pub fn new() -> Self {
        BlockStateBuilder {
            properties: HashMap::new()
        }
    }

    /// Register a property to add to the built states. Properties are indexed
    /// by their name, so this method will panic if you add properties with the
    /// same name.
    pub fn prop<T, P>(mut self, property: &P) -> Self
        where
            T: PropertySerializable,
            P: Property<T>
    {

        match self.properties.entry(property.name()) {
            Entry::Occupied(_) => panic!("Property '{}' already registered.", property.name()),
            Entry::Vacant(v) => {
                let len = property.len();
                if len == 0 {
                    panic!("Property '{}' length is 0.", property.name());
                }
                v.insert((property.type_id(), len));
            }
        }

        self

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

        for (&name, &(type_id, length)) in &self.properties {
            states_count *= length as usize;
            properties_periods.push((name, type_id, length, 1usize));
        }

        if *uid as usize + states_count > u16::MAX as usize {
            panic!("Not enough states UIDs to store these block states.");
        }

        // We know that the states count is valid, so we allocate the block's shared data.
        let shared_data_arc = Arc::new(
            BlockSharedData::new(self.properties.len(), states_count)
        );

        // SAFETY: Shared data can be mutated because at this point the data is not
        //   changed by shared cloned Arc in this method.
        let shared_data = unsafe { util::mutate_ref(&*shared_data_arc) };
        let shared_properties = &mut shared_data.properties;
        let shared_states = &mut shared_data.states;

        let mut next_period = 1;
        for (i, (name, type_id, length, period)) in properties_periods.iter_mut().enumerate().rev() {
            *period = next_period;
            next_period *= *length as usize;
            shared_properties.insert(*name, SharedProperty {
                type_id: *type_id,
                index: i,
                length: *length,
                period: *period
            });
        }

        for i in 0..states_count {

            let mut state_properties = Vec::with_capacity(properties_periods.len());

            for (_, _, length, period) in properties_periods.iter() {
                state_properties.push(((i / *period) % (*length as usize)) as u8);
            }

            let state = Arc::new(BlockState {
                uid: *uid,
                reg_tid,
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

        if prop.type_id == property.type_id() {
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

    pub fn with<T, P>(&self, property: &P, value: T) -> Option<Arc<BlockState>>
        where
            T: PropertySerializable,
            P: Property<T>
    {

        // SAFETY: Shared data is weak, but we except it to exists.
        let data = self.shared_data.upgrade().unwrap();
        let prop = data.properties.get(&property.name())?;

        let new_value = property.encode(value)? as isize;
        let current_value = self.properties[prop.index] as isize;
        let value_diff = new_value - current_value;
        let neighbor_index = (self.index as isize + value_diff) as usize;

        Some(Arc::clone(&data.states[neighbor_index]))

    }

}
