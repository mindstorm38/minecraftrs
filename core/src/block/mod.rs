use std::collections::HashMap;
use std::any::{TypeId, Any};
use std::sync::{Arc, Weak};

mod state;
pub use state::*;
pub mod vanilla;


/// A basic block.
#[derive(Debug)]
pub struct Block {
    name: &'static str,
    states: Vec<Arc<BlockState>>,
    default_state: Weak<BlockState>
}

impl Block {

    pub fn new(
        name: &'static str,
        state_builder: BlockStateBuilder,
        reg_tid: TypeId,
        uid: &mut u16,
        states_by_uid: &mut HashMap<u16, Weak<BlockState>>
    ) -> Self {

        let states = state_builder.build(reg_tid, uid, states_by_uid);

        Block {
            name,
            default_state: Arc::downgrade(&states[0]),
            states,
        }

    }

    pub fn get_default_state(&self) -> Arc<BlockState> {
        // SAFETY: Unwrap should never panic if the object is not dropped.
        self.default_state.upgrade().unwrap()
    }

}


/// Trait to implement for all Blocks static registers, like the one generated by `blocks!` macro.
///
/// This requires the implementation of Any ('static) in order to use the TypeId to resolve
/// UID offset.
pub trait StaticBlocks: Any {
    fn get_state(&self, uid: u16) -> Option<Arc<BlockState>>;
    fn get_last_uid(&self) -> u16;
}


/// Effective block register, can be composed of multiple static registers for use in worlds.
pub struct Blocks {
    uid_offset_and_reg: Vec<(u16, &'static dyn StaticBlocks)>,
    uid_offset_by_reg_tid: HashMap<TypeId, u16>,
    next_uid_offset: u16
}

impl Blocks {

    pub fn new() -> Blocks {
        Blocks {
            uid_offset_and_reg: Vec::new(),
            uid_offset_by_reg_tid: HashMap::new(),
            next_uid_offset: 1 // 0 is reserved, like the null-ptr
        }
    }

    pub fn register<B: StaticBlocks>(&mut self, reg: &'static B) {
        self.uid_offset_and_reg.insert(0, (self.next_uid_offset, reg));
        self.uid_offset_by_reg_tid.insert(reg.type_id(), self.next_uid_offset);
        self.next_uid_offset += reg.get_last_uid() + 1;
    }

    pub fn get_state_uid(&self, state: &BlockState) -> u16 {
        if let Some(&offset) = self.uid_offset_by_reg_tid.get(&state.reg_tid) {
            offset + state.uid
        } else {
            panic!("This BlockState is not registered in this Blocks register, first register its static register.");
        }
    }

    pub fn get_state(&self, uid: u16) -> Option<Arc<BlockState>> {
        if uid > 0 {
            for &(uid_offset, reg) in &self.uid_offset_and_reg {
                if uid >= uid_offset {
                    return reg.get_state(uid - uid_offset);
                }
            }
        }
        None
    }

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
            states_by_uid: std::collections::HashMap<u16, std::sync::Weak<$crate::block::BlockState>>,
            last_uid: u16,
            $( pub $block_id: $crate::block::Block ),*
        }

        impl $struct_id {
            fn load() -> Self {

                use std::collections::HashMap;
                use $crate::block::{Block, BlockStateBuilder};

                let mut uid = 0;
                let tid = std::any::TypeId::of::<Self>();
                let mut states_by_uid = HashMap::new();

                Self {
                    $( $block_id: Block::new(
                        $block_name,
                        BlockStateBuilder::new() $($( .prop(&$prop_const) )*)?,
                        tid,
                        &mut uid,
                        &mut states_by_uid
                    ), )*
                    last_uid: uid,
                    states_by_uid
                }

            }
        }

        impl $crate::block::StaticBlocks for $struct_id {

            fn get_state(&self, uid: u16) -> Option<std::sync::Arc<$crate::block::BlockState>> {
                self.states_by_uid.get(&uid).map(|weak_state| weak_state.upgrade().unwrap())
            }

            fn get_last_uid(&self) -> u16 {
                self.last_uid
            }

        }

        #[allow(non_upper_case_globals)]
        pub static $static_id: once_cell::sync::Lazy<$struct_id> = once_cell::sync::Lazy::new(|| $struct_id::load());

    };
}
