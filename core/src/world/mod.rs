use std::collections::HashMap;

use crate::block::WorkBlocks;
use crate::biome::WorkBiomes;

pub mod level;
pub mod chunk;
pub mod loader;

use level::Level;
use std::sync::{Arc, Weak, RwLock};


// TODO: There is an issue with "back pointers" to parent structures, because we can in this case
//  get mutate reference to a Chunk, and from this chunk get back immutable reference to the Level.
//  With this reference to Level we can therefore get an immutable reference to another Chunk while
//  mutating the first one.
//
//  let level_mut: &mut Level = ...;
//  let chunk_ref: &mut Chunk = level_mut.get_chunk_mut().unwrap();
//  let level_ref: &Level = chunk_ref.get_level();
//
//  This last line is ILLEGAL, because we already mutated level_mut.
//
//  To fix this we can put all world data in a sub structure and store it in an Arc<_>,
//  or an Arc<Mutex<_>>


/// Main storage for a world, contains blocks, biomes registers
/// and levels.
///
/// World runtime must be implemented externally to this structure.
pub struct World {
    /// Weak counted reference to this structure, this implies that
    /// this structure must be owned by an `Arc<RwLock<_>>`. This is
    /// used internally when building `Level`s.
    this: Weak<RwLock<World>>,
    /// Actual blocks register.
    blocks: WorkBlocks<'static>,
    /// Actual biomes register.
    biomes: WorkBiomes<'static>,
    /// Maps of levels owned by this world, associated to their IDs.
    levels: HashMap<&'static str, Arc<RwLock<Level>>>,
}

impl World {

    pub fn new(blocks: WorkBlocks<'static>, biomes: WorkBiomes<'static>) -> Arc<RwLock<World>> {

        let ret = Arc::new(RwLock::new(World {
            this: Weak::new(),
            blocks,
            biomes,
            levels: HashMap::new()
        }));

        ret.write().unwrap().this = Arc::downgrade(&ret);
        ret

    }

    pub fn new_vanilla() -> Arc<RwLock<World>> {
        Self::new(WorkBlocks::new_vanilla(), WorkBiomes::new_vanilla())
    }

    pub fn get_blocks(&self) -> &WorkBlocks<'static> {
        &self.blocks
    }

    pub fn get_biomes(&self) -> &WorkBiomes<'static> {
        &self.biomes
    }

}
