use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use std::any::TypeId;
use std::rc::Rc;
use lazy_static::lazy_static;

mod state;
mod vanilla;

pub use state::*;


/// A basic block.
#[derive(Debug)]
pub struct Block {
    name: &'static str,
    states: Vec<Rc<RefCell<BlockState>>>,
    default_state: Rc<RefCell<BlockState>>
}


impl Block {

    pub fn new(name: &'static str, state_builder: BlockStateBuilder) -> Self {
        let states = state_builder.build();
        Block {
            name,
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


pub trait BlockDefinitions {
    fn register(&self);
}


#[macro_export]
macro_rules! blocks {
    ($struct_id:ident $static_id:ident [
        $(
            $block_id:ident $block_name:literal $([ $($prop_const:ident),* ])?
        ),*
    ]) => {

        #[allow(non_snake_case)]
        pub struct $struct_id {
            $( pub $block_id: $crate::block::Block ),*
        }

        impl $struct_id {
            fn load() -> Self {
                Self {
                    $(
                        $block_id: $crate::block::Block::new($block_name, {
                            $crate::block::BlockStateBuilder::new()
                            $($( .prop(&$prop_const) )*)?
                        })
                    ),*
                }
            }
        }

        lazy_static! {
            pub static ref $static_id: $struct_id = $struct_id::load();
        };

    };
}
