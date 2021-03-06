use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use std::any::TypeId;
use std::rc::Rc;

use crate::res::Registrable;

#[deprecated]
mod registry;
pub use registry::*;

mod vanilla;
pub use vanilla::*;

mod property;
pub use property::*;


/// A basic block.
// #[derive(Debug)]
pub struct Block {
    name: &'static str,
    #[deprecated]
    id: u16,
    states: Vec<Rc<RefCell<BlockState>>>,
    default_state: Rc<RefCell<BlockState>>
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

    #[deprecated]
    pub fn new(name: &'static str, id: u16) -> Self {
        let states = BlockStateBuilder::new().build(&mut 0);
        Block {
            name,
            id,
            default_state: Rc::clone(&states[0]),
            states,
        }
    }

    pub fn from_builder(name: &'static str, state_builder: BlockStateBuilder, uid: &mut u16) -> Self {
        let states = state_builder.build(uid);
        Block {
            name,
            id: 0,
            default_state: Rc::clone(&states[0]),
            states,
        }
    }

    pub fn get_default_state(&self) -> Ref<BlockState> {
        self.default_state.borrow()
    }

    pub fn get_default_state_mut(&mut self) -> RefMut<BlockState> {
        self.default_state.borrow_mut()
    }

}


/// Represent a particular state of a block, this block state also know
/// all its neighbors by their properties and values.
///
/// To build states, use `BlockStateContainerBuilder` and add all wanted
/// properties.
#[derive(Debug)]
pub struct BlockState {
    uid: u16,
    properties: HashMap<&'static str, (TypeId, u8)>,
    neighbors: HashMap<(&'static str, TypeId, u8), Rc<RefCell<BlockState>>>
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
    /// block states know their neighbors by property and values. The given uid reference
    /// is set and incremented for each state.
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

            if let Some(new_uid) = uid.checked_add(1) {
                *uid = new_uid;
            } else {
                panic!("Block state uid overflown (> {})", uid);
            }

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


#[macro_export]
macro_rules! def_blocks {
    ($struct_id:ident [
        $(
            $block_id:ident $block_name:literal $([ $($prop_const:ident),* ])?
        ),*
    ]) => {

        #[allow(non_snake_case)]
        pub struct $struct_id {
            $( pub $block_id: $crate::block::Block ),*
        }

        impl $struct_id {
            pub fn load() -> Self {
                let mut uid = 1;
                Self {
                    $(
                        $block_id: $crate::block::Block::from_builder($block_name, {
                            $crate::block::BlockStateBuilder::new()
                            $($( .prop(&$prop_const) )*)?
                        }, &mut uid)
                    ),*
                }
            }
        }

    };
}
