use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::any::{TypeId, Any};
use std::cell::{RefCell, Ref};
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


pub struct BlockState {
    container: Rc<RefCell<BlockStateContainer>>,
    properties: HashMap<&'static str, (TypeId, u8)>,
    neighbors: HashMap<(&'static str, TypeId, u8), usize>
}


pub struct BlockStateContainer {
    states: Vec<BlockState>
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

    pub fn build(self) -> Rc<RefCell<BlockStateContainer>> {

        let mut groups = vec![vec![]];
        let mut future_groups = vec![];

        for (_, values) in self.properties.values() {
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

        let mut container = Rc::new(RefCell::new(
            BlockStateContainer {
                states: Vec::new()
            }
        ));

        for group in groups {

            let mut properties = HashMap::new();

            for (i, (&name, (type_id, _))) in self.properties.iter().enumerate() {
                properties.insert(name, (*type_id, group[i]));
            }

            container.states.push(BlockState {
                container: Rc::clone(&container),
                properties,
                neighbors: Default::default()
            })

        }

        for state in &mut states {



        }

        container

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

    /*pub fn with<T, P>(&self, property: &P, value: T) -> &BlockState
    where
        T: Copy,
        P: Property<T>
    {

    }*/

}
