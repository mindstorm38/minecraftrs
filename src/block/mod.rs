use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use std::any::TypeId;
use std::rc::Rc;

use crate::res::Registrable;

mod registry;
pub use registry::*;

pub mod property;
use property::{Property, IntProperty};


pub static PROP_LIQUID_LEVEL: IntProperty = IntProperty("level", 15);


/// A basic block.
#[derive(Debug)]
pub struct Block {
    name: &'static str,
    id: u16
}

impl Registrable<u16> for Block {
    fn get_name(&self) -> &'static str { self.name }
    fn get_id(&self) -> u16 { self.id }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("{}/{}", self.id, self.name))
    }
}

impl Block {

    pub fn new(name: &'static str, id: u16) -> Self {
        Block {
            name,
            id
        }
    }

}


/// To keep states safe, do not extract them from their states vectors.
pub struct BlockState {
    uid: u16,
    properties: HashMap<&'static str, (TypeId, u8)>,
    neighbors: HashMap<(&'static str, TypeId, u8), Rc<RefCell<BlockState>>>
}


pub struct BlockStateContainerBuilder {
    properties: HashMap<&'static str, (TypeId, Vec<u8>)>
}


impl BlockStateContainerBuilder {

    pub fn new() -> Self {
        BlockStateContainerBuilder {
            properties: HashMap::new()
        }
    }

    pub fn add<T, P>(&mut self, property: &P)
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
    }

    pub fn build(self, uid: &mut u16) -> Vec<Rc<RefCell<BlockState>>> {

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

        for group in &groups {

            let mut group_properties = HashMap::new();

            for (prop_idx, &(name, type_id, _)) in properties.iter().enumerate() {
                group_properties.insert(name, (type_id, group[prop_idx]));
            }

            states.push(Rc::new(RefCell::new(BlockState {
                uid: *uid,
                properties: group_properties,
                neighbors: HashMap::new()
            })));

            *uid += 1;

        }

        for (idx, state) in states.iter().enumerate() {
            for (neighbor_idx, neighbor_group) in groups.iter().enumerate() {
                if idx != neighbor_idx {
                    for (prop_idx, &neighbor_prop) in neighbor_group.iter().enumerate() {
                        let (prop_name, type_id, _) = properties[prop_idx];
                        state.borrow_mut().neighbors.insert((prop_name, type_id, neighbor_prop), Rc::clone(&states[neighbor_idx]));
                    }
                }
            }
        }

        states

    }

}


impl BlockState {

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

    pub fn with<T, P>(&self, property: &P, value: T) -> Option<Ref<BlockState>>
    where
        T: Copy,
        P: Property<T>
    {

        self.neighbors.get(&(property.get_name(), property.type_id(), property.encode_prop(value)))
            .map(|state| state.borrow())

    }

    pub fn with_mut<T, P>(&self, property: &P, value: T) -> Option<RefMut<BlockState>>
    where
        T: Copy,
        P: Property<T>
    {

        self.neighbors.get(&(property.get_name(), property.type_id(), property.encode_prop(value)))
            .map(|state| state.borrow_mut())

    }

}
