use std::marker::PhantomPinned;
use std::collections::HashMap;
use std::ptr::NonNull;
use std::fmt::Debug;
use std::any::Any;
use std::pin::Pin;

use crate::util::generic::{RwGenericMap, GuardedRef, GuardedMut};
use crate::util::UidGenerator;

mod state;
mod property;
pub use state::*;
pub use property::*;

#[cfg(feature = "vanilla_blocks")]
pub mod vanilla;
#[cfg(feature = "vanilla_blocks")]
pub mod legacy;


/// A basic block defined by a name, its states, properties or extensions.
///
/// A Block can only live in heap memory through a pined box because its states
/// contains a back reference to it. Block and BlockState must not be mutated,
/// only extensions can be 'interior-mutated'.
#[derive(Debug)]
pub struct Block {
    uid: u32,
    name: &'static str,
    states: BlockStorage,
    extensions: RwGenericMap,
    marker: PhantomPinned
}

// We enforce Send + Sync because we know that these pointers will never
// be mutated but only immutably referenced.
unsafe impl Send for Block {}
unsafe impl Sync for Block {}


/// Internal enumeration to avoid allocation over-head for single block. This
/// allows blocks with no properties to avoid allocating a `Vec` and a `HashMap`.
#[derive(Debug)]
enum BlockStorage {
    /// Storage for a single state.
    Single(BlockState),
    /// Storage when there is single or multiple properties. This type of storage
    /// implies that all owned states must have BlockStateProperties::Some.
    Complex {
        states: Vec<BlockState>,
        properties: HashMap<&'static str, SharedProperty>,
        default_state_index: usize
    }
}


/// Made for static definitions of all properties of a block.
pub enum BlockSpec {
    /// For blocks with no properties, they have a **single** state.
    Single,
    /// For blocks with some properties, requires a slice to a static array of properties
    /// references. Use the `blocks_specs!` macro to generate such arrays.
    Complex(&'static [&'static dyn UntypedProperty]),
    /// Same a `Complex`, but with a callback function used to set the default block state.
    ComplexWithDefault(&'static [&'static dyn UntypedProperty], fn(&BlockState) -> &BlockState)
}


impl Block {

    pub fn new(name: &'static str, spec: BlockSpec) -> Pin<Box<Block>> {

        // Static UID generator, common to all blocks.
        static UID: UidGenerator = UidGenerator::new();

        // Internal function to generate new BlockStorage from properties,
        // if there are no properties, BlockStorage::Single is returned.
        fn new_storage(properties: &'static [&'static dyn UntypedProperty]) -> BlockStorage {
            if properties.is_empty() {
                BlockStorage::Single(BlockState::build_singleton())
            } else {

                let (
                    properties,
                    states
                ) = BlockState::build_complex(properties);

                BlockStorage::Complex {
                    states,
                    properties,
                    default_state_index: 0
                }

            }
        }

        // The default supplier is obviously useless for BlockStorage::Single.
        let mut default_supplier = None;

        let states = match spec {
            BlockSpec::Single => BlockStorage::Single(BlockState::build_singleton()),
            BlockSpec::Complex(properties) => new_storage(properties),
            BlockSpec::ComplexWithDefault(properties, fun) => {
                default_supplier = Some(fun);
                new_storage(properties)
            }
        };

        let mut block = Box::pin(Block {
            uid: UID.next(),
            name,
            states,
            extensions: RwGenericMap::new(),
            marker: PhantomPinned
        });

        unsafe {

            let block_ref = NonNull::from(&*block);
            let block_mut = block.as_mut().get_unchecked_mut();

            match &mut block_mut.states {
                BlockStorage::Single(state) => state.set_block(block_ref),
                BlockStorage::Complex { states, default_state_index, .. } => {
                    for state in states.iter_mut() {
                        state.set_block(block_ref);
                    }
                    if let Some(default_supplier) = default_supplier {
                         *default_state_index = default_supplier(&states[0]).get_index() as usize;
                    }
                }
            }

        }

        block

    }

    /// Get the unique ID of this block, this is unique for the process.
    /// This UID is not used for any save operation, for saving purpose,
    /// use `WorkBlocks`.
    pub fn get_uid(&self) -> u32 {
        self.uid
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_default_state(&self) -> &BlockState {
        match &self.states {
            BlockStorage::Single(state) => state,
            BlockStorage::Complex {
                states,
                default_state_index, ..
            } => &states[*default_state_index]
        }
    }

    pub fn get_states(&self) -> &[BlockState] {
        match &self.states {
            BlockStorage::Single(state) => std::slice::from_ref(state),
            BlockStorage::Complex { states, .. } => &states[..]
        }
    }

    pub fn add_ext<E: Any + Sync + Send>(&self, ext: E) {
        self.extensions.add(ext);
    }

    pub fn get_ext<E: Any + Sync + Send>(&self) -> Option<GuardedRef<E>> {
        self.extensions.get()
    }

    pub fn get_ext_mut<E: Any + Sync + Send>(&self) -> Option<GuardedMut<E>> {
        self.extensions.get_mut()
    }

    /// Internal method for neighbor and values resolution of `BlockState`.
    fn get_shared_prop(&self, name: &'static str) -> Option<&SharedProperty> {
        match &self.states {
            BlockStorage::Single(_) => None,
            BlockStorage::Complex {
                properties, ..
            } => properties.get(&name)
        }
    }

    /// Internal method for Debug implementation of `BlockState`.
    fn get_shared_props(&self) -> Option<&HashMap<&'static str, SharedProperty>> {
        match &self.states {
            BlockStorage::Single(_) => None,
            BlockStorage::Complex {
                properties, ..
            } => Some(properties)
        }
    }

    /// Internal method for `BlockState` to get a state a specific index.
    fn get_state_unchecked(&self, index: usize) -> &BlockState {
        match &self.states {
            BlockStorage::Single(state) => {
                debug_assert!(index == 0, "index != 0 with BlockStorage::Single");
                state
            },
            BlockStorage::Complex { states, .. } => &states[index]
        }
    }

}


/// Trait to implement for all Blocks static registers, like the one generated by `blocks!` macro.
///
/// This requires the implementation of Any ('static) in order to use the TypeId to resolve
/// UID offset.
pub trait StaticBlocks {
    fn iter_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Pin<Box<Block>>> + 'a>;
    fn blocks_count(&self) -> usize;
    fn states_count(&self) -> usize;
}


/// Working blocks' registry, use this structure to add individual blocks to the register.
/// This registry maps unique blocks and block states IDs to save IDs (SID).
pub struct WorkBlocks<'a> {
    next_sid: u16,
    blocks_to_sid: HashMap<u32, u16>,
    sid_to_states: Vec<&'a BlockState>
}

#[cfg(feature = "vanilla_blocks")]
impl WorkBlocks<'static> {

    pub fn new_vanilla() -> Result<WorkBlocks<'static>, ()> {
        let mut r = Self::new();
        r.register_static(&*vanilla::VanillaBlocks)?;
        Ok(r)
    }

}

impl<'a> WorkBlocks<'a> {

    pub fn new() -> WorkBlocks<'a> {
        WorkBlocks {
            next_sid: 0,
            blocks_to_sid: HashMap::new(),
            sid_to_states: Vec::new()
        }
    }

    pub fn register(&mut self, block: &'a Pin<Box<Block>>) -> Result<(), ()> {
        let block = &**block;
        let states = block.get_states();
        let states_count = states.len();
        let uid = self.next_sid;
        self.next_sid = uid.checked_add(states_count as u16).ok_or(())?;
        self.blocks_to_sid.insert(block.uid, uid);
        self.sid_to_states.reserve(states_count);
        for state in states {
            self.sid_to_states.push(state);
        }
        Ok(())
    }

    pub fn register_static(&mut self, static_blocks: &'a Pin<Box<impl StaticBlocks>>) -> Result<(), ()> {
        self.blocks_to_sid.reserve(static_blocks.blocks_count());
        self.sid_to_states.reserve(static_blocks.states_count());
        for block in static_blocks.iter_blocks() {
            self.register(block)?;
        }
        Ok(())
    }

    pub fn get_sid_from(&self, state: &BlockState) -> Option<u16> {
        let block_uid = state.get_block().get_uid();
        let block_offset = *self.blocks_to_sid.get(&block_uid)?;
        Some(block_offset + state.get_index())
    }

    pub fn get_state_from(&self, sid: u16) -> Option<&'a BlockState> {
        Some(*self.sid_to_states.get(sid as usize)?)
    }

    pub fn blocks_count(&self) -> usize {
        self.blocks_to_sid.len()
    }

    pub fn states_count(&self) -> usize {
        self.sid_to_states.len()
    }

}

#[macro_export]
macro_rules! blocks_specs {
    ($($v:vis $id:ident: [$($prop_const:ident),+];)*) => {
        $(
            $v static $id: [&'static dyn $crate::block::UntypedProperty; $crate::count!($($prop_const)+)] = [
                $(&$prop_const),+
            ];
        )*
    };
}

#[macro_export]
macro_rules! blocks {
    ($struct_id:ident $static_id:ident [
        $(
            $block_id:ident $block_name:literal $($spec_id:ident)?
        ),*
        $(,)?
    ]) => {

        #[allow(non_snake_case)]
        pub struct $struct_id {
            blocks: Vec<std::ptr::NonNull<std::pin::Pin<Box<$crate::block::Block>>>>,
            states_count: usize,
            $( pub $block_id: std::pin::Pin<Box<$crate::block::Block>>, )*
            _marker: std::marker::PhantomPinned
        }

        impl $struct_id {
            pub fn load() -> std::pin::Pin<Box<Self>> {

                use $crate::block::Block;

                use std::marker::PhantomPinned;
                use std::ptr::NonNull;
                use std::pin::Pin;

                let mut blocks_count = 0;
                let mut states_count = 0;

                let mut inc = |b: Pin<Box<Block>>| {
                    blocks_count += 1;
                    states_count += b.get_states().len();
                    b
                };

                let mut reg = Box::pin(Self {
                    $($block_id: inc(Block::new($block_name, $crate::inner_blocks_spec!($($spec_id)?))),)*
                    blocks: Vec::with_capacity(blocks_count),
                    states_count,
                    _marker: PhantomPinned
                });

                unsafe {
                    let reg_mut = reg.as_mut().get_unchecked_mut();
                    let reg_blocks = &mut reg_mut.blocks;
                    $(reg_blocks.push(NonNull::from(&reg_mut.$block_id));)*
                }

                reg

            }
        }

        // Enforce Send/Sync because NonNull are pointing to pined box content.
        unsafe impl Send for $struct_id {}
        unsafe impl Sync for $struct_id {}

        impl $crate::block::StaticBlocks for $struct_id {

            fn iter_blocks<'a>(&'a self) -> Box<dyn Iterator<Item=&'a std::pin::Pin<Box<$crate::block::Block>>> + 'a> {
                Box::new(self.blocks.iter().map(|ptr| unsafe { ptr.as_ref() }))
            }

            fn blocks_count(&self) -> usize {
                self.blocks.len()
            }

            fn states_count(&self) -> usize {
                self.states_count
            }

        }

        #[allow(non_upper_case_globals)]
        pub static $static_id: once_cell::sync::Lazy<std::pin::Pin<Box<$struct_id>>> = once_cell::sync::Lazy::new(|| $struct_id::load());

    };
}

#[macro_export]
macro_rules! inner_blocks_spec {
    () => { $crate::block::BlockSpec::Single };
    ($spec_id:ident) => { $crate::block::BlockSpec::Complex(&$spec_id) }
}
