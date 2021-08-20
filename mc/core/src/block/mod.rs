use std::collections::HashMap;
use std::ptr::NonNull;
use std::fmt::Debug;

use once_cell::sync::OnceCell;
use crate::util::OpaquePtr;

mod state;
mod property;
mod behaviour;

pub use state::*;
pub use property::*;
pub use behaviour::*;


/// A basic block defined by a name, its states and properties. This block structure
/// is made especially for static definition, its states are computed lazily and
/// almost all method requires a self reference with static lifetime.
#[derive(Debug)]
pub struct Block {
    name: &'static str,
    spec: BlockSpec,
    states: OnceCell<BlockStorage>,
}


/// The type of hashable value that can represent a block as a map key.
/// See `Block::get_key`, its only usable for statically defined blocks.
pub type BlockKey = OpaquePtr<Block>;


/// Internal enumeration to avoid allocation over-head for single block. This allows
/// blocks with no properties to avoid allocating a `Vec` and a `HashMap`.
#[derive(Debug)]
enum BlockStorage {
    /// Storage for a single state.
    Single(BlockState),
    /// Storage when there is single or multiple properties. This type of storage
    /// implies that all owned states must have BlockStateProperties::Some.
    /// By using this storage you assert that properties map is not empty.
    Complex {
        states: Vec<BlockState>,
        properties: HashMap<&'static str, SharedProperty>,
        default_state_index: usize
    }
}


/// Made for static definitions of all properties of a block.
#[derive(Debug)]
pub enum BlockSpec {
    /// For blocks with no properties, they have a **single** state.
    Single,
    /// For blocks with some properties, requires a slice to a static array of properties
    /// references. Use the `blocks_specs!` macro to generate such arrays.
    Complex(&'static [&'static dyn UntypedProperty]),
    // /// Same a `Complex`, but with a callback function used to set the default block state.
    // ComplexWithDefault(&'static [&'static dyn UntypedProperty], fn(&BlockState) -> &BlockState)
}


impl Block {

    /// Construct a new block, this method should be used to define blocks statically.
    /// The preferred way of defining static blocks is to use the `blocks!` macro.
    pub const fn new(name: &'static str, spec: BlockSpec) -> Self {
        Self {
            name,
            spec,
            states: OnceCell::new()
        }
    }

    #[inline]
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn get_key(&'static self) -> BlockKey {
        OpaquePtr::new(self)
    }

    fn get_storage(&'static self) -> &'static BlockStorage {
        self.states.get_or_init(|| self.make_storage())
    }

    fn make_storage(&'static self) -> BlockStorage {

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

        // let mut default_supplier = None;

        let mut storage = match self.spec {
            BlockSpec::Single => BlockStorage::Single(BlockState::build_singleton()),
            BlockSpec::Complex(properties) => new_storage(properties),
            /*BlockSpec::ComplexWithDefault(properties, fun) => {
                default_supplier = Some(fun);
                new_storage(properties)
            }*/
        };

        let block_ptr = NonNull::from(self);

        unsafe {

            match &mut storage {
                BlockStorage::Single( state) => {
                    state.set_block(block_ptr);
                },
                BlockStorage::Complex {
                    states,
                    default_state_index, ..
                } => {
                    for state in states {
                        state.set_block(block_ptr);
                    }
                    /*if let Some(default_supplier) = default_supplier {
                        *default_state_index = default_supplier(&states[0]).get_index() as usize;
                    }*/
                }
            }

        }

        storage

    }

    #[inline]
    pub fn get_default_state(&'static self) -> &'static BlockState {
        self.get_storage().get_default_state()
    }

    #[inline]
    pub fn get_states(&'static self) -> &'static [BlockState] {
        self.get_storage().get_states()
    }

}


impl PartialEq for &'static Block {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}

impl Eq for &'static Block {}


impl BlockStorage {

    pub fn get_default_state(&self) -> &BlockState {
        match self {
            BlockStorage::Single(state) => state,
            BlockStorage::Complex {
                states,
                default_state_index, ..
            } => &states[*default_state_index]
        }
    }

    pub fn get_states(&self) -> &[BlockState] {
        match self {
            BlockStorage::Single(state) => std::slice::from_ref(state),
            BlockStorage::Complex { states, .. } => &states[..]
        }
    }

    /// Internal method for neighbor and values resolution of `BlockState`.
    fn get_shared_prop(&self, name: &str) -> Option<&SharedProperty> {
        match self {
            BlockStorage::Single(_) => None,
            BlockStorage::Complex {
                properties, ..
            } => properties.get(name)
        }
    }

    /// Internal method for Debug implementation of `BlockState` and values iteration.
    /// None is returned if there is no properties and the block has a single state.
    fn get_shared_props(&self) -> Option<&HashMap<&'static str, SharedProperty>> {
        match self {
            BlockStorage::Single(_) => None,
            BlockStorage::Complex {
                properties, ..
            } => Some(properties)
        }
    }

    /// Internal method for `BlockState` to get a state a specific index.
    fn get_state_unchecked(&self, index: usize) -> &BlockState {
        match self {
            BlockStorage::Single(state) => {
                debug_assert!(index == 0, "index != 0 with BlockStorage::Single");
                state
            },
            BlockStorage::Complex { states, .. } => &states[index]
        }
    }

}


/// This is a global blocks palette, it is used in chunk storage to store block states.
/// It allows you to register individual blocks in it as well as static blocks arrays
/// defined using the macro `blocks!`.
pub struct GlobalBlocks {
    next_sid: u32,
    block_to_sid: HashMap<BlockKey, u32>,
    sid_to_state: Vec<&'static BlockState>,
    name_to_block: HashMap<&'static str, &'static Block>
}

impl GlobalBlocks {

    pub fn new() -> Self {
        Self {
            next_sid: 0,
            block_to_sid: HashMap::new(),
            sid_to_state: Vec::new(),
            name_to_block: HashMap::new()
        }
    }

    pub fn with_static(static_blocks: &[&'static Block]) -> Result<Self, ()> {
        let mut blocks = Self::new();
        blocks.register_static(static_blocks)?;
        Ok(blocks)
    }

    /// Register a single block to this palette, returns `Err` if no more save ID (SID) is
    /// available, `Ok` is returned if successful, if a block was already in the palette
    /// it also returns `Ok`.
    pub fn register(&mut self, block: &'static Block) -> Result<(), ()> {

        let states = block.get_states();
        let states_count = states.len();

        let sid = self.next_sid;
        let next_sid = sid.checked_add(states_count as u32).ok_or(())?;

        if let None = self.block_to_sid.insert(block.get_key(), sid) {

            self.next_sid = next_sid;

            self.name_to_block.insert(block.name, block);
            self.sid_to_state.reserve(states_count);
            for state in states {
                self.sid_to_state.push(state);
            }

        }

        Ok(())

    }

    pub fn register_static(&mut self, static_blocks: &[&'static Block]) -> Result<(), ()> {
        let count = static_blocks.len();
        self.block_to_sid.reserve(count);
        self.name_to_block.reserve(count);
        for &block in static_blocks {
            self.register(block)?;
        }
        Ok(())
    }

    /// Get the save ID from the given state.
    pub fn get_sid_from(&self, state: &'static BlockState) -> Option<u32> {
        let block_offset = *self.block_to_sid.get(&state.get_block().get_key())?;
        Some(block_offset + state.get_index() as u32)
    }

    /// Get the block state from the given save ID.
    pub fn get_state_from(&self, sid: u32) -> Option<&'static BlockState> {
        Some(*self.sid_to_state.get(sid as usize)?)
    }

    /// Get the default state from the given block name.
    pub fn get_block_from_name(&self, name: &str) -> Option<&'static Block> {
        self.name_to_block.get(name).cloned()
    }

    /// Return true if the palette contains the given block state.
    pub fn has_state(&self, state: &'static BlockState) -> bool {
        self.block_to_sid.contains_key(&state.get_block().get_key())
    }

    /// Check if the given state is registered in this palette, `Ok` is returned if true, in
    /// the other case `Err` is returned with the error created by the given `err` closure.
    pub fn check_state<E>(&self, state: &'static BlockState, err: impl FnOnce() -> E) -> Result<&'static BlockState, E> {
        if self.has_state(state) { Ok(state) } else { Err(err()) }
    }

    pub fn blocks_count(&self) -> usize {
        self.block_to_sid.len()
    }

    pub fn states_count(&self) -> usize {
        self.sid_to_state.len()
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
    ($global_vis:vis $static_id:ident $namespace:literal [
        $($block_id:ident $block_name:literal $($spec_id:ident)?),*
        $(,)?
    ]) => {

        $($global_vis static $block_id: $crate::block::Block = $crate::block::Block::new(
            concat!($namespace, ':', $block_name),
            $crate::inner_blocks_spec!($($spec_id)?)
        );)*

        $global_vis static $static_id: [&'static $crate::block::Block; $crate::count!($($block_id)*)] = [
            $(&$block_id),*
        ];

    };
}

#[macro_export]
macro_rules! inner_blocks_spec {
    () => { $crate::block::BlockSpec::Single };
    ($spec_id:ident) => { $crate::block::BlockSpec::Complex(&$spec_id) }
}
