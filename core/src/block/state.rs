use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::{Weak, Arc};

use super::Block;
use crate::util;


/// Trait for all properties stored in a block state.
pub trait Property<T: Copy>: Any {
    fn get_name(&self) -> &'static str;
    fn iter_values(&self) -> Box<dyn Iterator<Item=T>>;
    fn encode_prop(&self, value: T) -> u8;
    fn decode_prop(&self, raw: u8) -> Option<T>;
}


pub struct BoolProperty(pub &'static str);

impl Property<bool> for BoolProperty {

    fn get_name(&self) -> &'static str {
        self.0
    }

    fn iter_values(&self) -> Box<dyn Iterator<Item = bool>> {
        static VALUES: [bool; 2] = [false, true];
        Box::new(VALUES.iter().copied())
    }

    fn encode_prop(&self, value: bool) -> u8 {
        if value { 1 } else { 0 }
    }

    fn decode_prop(&self, raw: u8) -> Option<bool> {
        Some(raw != 0)
    }

}


pub struct IntProperty(pub &'static str, pub u8);

impl Property<u8> for IntProperty {

    fn get_name(&self) -> &'static str {
        self.0
    }

    fn iter_values(&self) -> Box<dyn Iterator<Item = u8>> {
        Box::new((0..self.1).into_iter())
    }

    fn encode_prop(&self, value: u8) -> u8 {
        value
    }

    fn decode_prop(&self, raw: u8) -> Option<u8> {
        if raw < self.1 {
            Some(raw)
        } else {
            None
        }
    }

}


pub struct EnumProperty<T: 'static + Copy + Eq>(pub &'static str, pub &'static [T]);

impl<T> Property<T> for EnumProperty<T>
where
    T: 'static + Copy + Eq
{

    fn get_name(&self) -> &'static str {
        self.0
    }

    fn iter_values(&self) -> Box<dyn Iterator<Item=T>> {
        Box::new(self.1.iter().copied())
    }

    fn encode_prop(&self, value: T) -> u8 {
        self.1.iter().position(|v| *v == value).unwrap() as u8
    }

    fn decode_prop(&self, raw: u8) -> Option<T> {
        self.1.get(raw as usize).copied()
    }

}


/// Represent a particular state of a block, this block state also know
/// all its neighbors by their properties and values.
///
/// To build states, use `BlockStateContainerBuilder` and add all wanted
/// properties.
#[derive(Debug)]
pub struct BlockState {
    pub(crate) uid: u16,
    pub(crate) reg_tid: TypeId,
    owner: Weak<Block>,
    properties: HashMap<&'static str, (TypeId, u8)>,
    neighbors: HashMap<(&'static str, TypeId, u8), Weak<BlockState>>
}


/// A builder used to build all block states according to a list of properties.
///
/// First register your properties using the `prop` then build the states vec
/// using `build`.
pub struct BlockStateBuilder {
    properties: HashMap<&'static str, (TypeId, Vec<u8>)>
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
            T: Copy,
            P: Property<T>
    {

        match self.properties.entry(property.get_name()) {
            Entry::Occupied(_) => panic!("Property '{}' already registered.", property.get_name()),
            Entry::Vacant(v) => {

                let values = property.iter_values()
                    .map(|val| property.encode_prop(val))
                    .collect::<Vec<u8>>();

                if values.is_empty() {
                    panic!("No property values returned by property '{}'.", property.get_name());
                }

                v.insert((property.type_id(), values));

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
    pub fn build(self, reg_tid: TypeId, uid: &mut u16) -> Vec<Arc<BlockState>> {

        // Move properties from the structure and convert to a linear properties vec
        let properties: Vec<(&'static str, TypeId, Vec<u8>)> = self.properties.into_iter()
            .map(|(name, (type_id, values))| (name, type_id, values))
            .collect();

        // Create all possible groups of properties values
        let mut groups = vec![vec![]];
        let mut future_groups = vec![];

        for (_, _, values) in &properties {
            for group in &groups {
                for &value in values {
                    let mut new_group = group.clone();
                    new_group.push(value);
                    future_groups.push(new_group);
                }
            }
            groups.clear();
            groups.extend_from_slice(&future_groups[..]);
            future_groups.clear();
        }

        // Build every state according to built groups
        let mut states = Vec::new();
        let mut uid_overflowed = false;

        for group in &groups {

            if uid_overflowed {
                // Delaying the panic by one iteration in order to
                // avoid panicking if the last state have the max UID.
                panic!("State UID overflown (> {})", *uid);
            }

            let mut group_properties = HashMap::new();

            for (prop_idx, &(name, type_id, _)) in properties.iter().enumerate() {
                group_properties.insert(name, (type_id, group[prop_idx]));
            }

            states.push(Arc::new(BlockState {
                uid: *uid,
                reg_tid,
                owner: Weak::new(),
                properties: group_properties,
                neighbors: HashMap::new()
            }));

            if let Some(new_uid) = uid.checked_add(1) {
                *uid = new_uid;
            } else {
                uid_overflowed = true;
            }

        }

        for (idx, state) in states.iter().enumerate() {
            for (neighbor_idx, neighbor_group) in groups.iter().enumerate() {
                if idx != neighbor_idx {
                    for (prop_idx, &neighbor_prop) in neighbor_group.iter().enumerate() {
                        let (prop_name, type_id, _) = properties[prop_idx];
                        unsafe {
                            // SAFETY: Neighbors states map is only set one time here, so I'm not
                            // using RefCell and this allows the trait to be Sync without Mutex.
                            util::mutate_ref(&state.neighbors).insert(
                                (prop_name, type_id, neighbor_prop),
                                Arc::downgrade(&states[neighbor_idx])
                            );
                        }
                    }
                }
            }
        }

        states

    }

}


impl BlockState {

    pub fn get_uid(&self) -> u16 {
        self.uid
    }

    /*pub(in crate::block) fn set_uid(&self, uid: u16) {
        // SAFETY: It's safe to set uid only at initialization of the blocks register,
        //         which need to be done before any manipulation of any world by any
        //         thread.
        unsafe {
            *(&self.uid as *const u16 as *mut u16) = uid;
        }
    }*/

    /// Get a block state property value if the property exists.
    pub fn get<T, P>(&self, property: &P) -> Option<T>
        where
            T: Copy,
            P: Property<T>
    {

        let (
            type_id,
            raw_value
        ) = self.properties.get(&property.get_name())?;

        if *type_id == property.type_id() {
            property.decode_prop(*raw_value)
        } else {
            None
        }

    }

    pub fn expect<T, P>(&self, property: &P) -> T
        where
            T: Copy,
            P: Property<T>
    {
        self.get(property).unwrap()
    }

    pub fn with<T, P>(&self, property: &P, value: T) -> Option<Arc<BlockState>>
        where
            T: Copy,
            P: Property<T>
    {

        self.neighbors.get(&(property.get_name(), property.type_id(), property.encode_prop(value)))
            .map(|state| state.upgrade().unwrap())

    }

}
