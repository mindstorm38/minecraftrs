use std::collections::HashMap;

use crate::block::WorkBlocks;
use crate::biome::WorkBiomes;

pub mod level;
pub mod chunk;
pub mod loader;

use level::Level;
use std::sync::{Arc, Weak, RwLock};
use std::collections::hash_map::Entry;


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

    #[cfg(all(feature = "vanilla_blocks", feature = "vanilla_biomes"))]
    pub fn new_vanilla() -> Arc<RwLock<World>> {
        Self::new(WorkBlocks::new_vanilla(), WorkBiomes::new_vanilla())
    }

    pub fn get_blocks(&self) -> &WorkBlocks<'static> {
        &self.blocks
    }

    pub fn get_biomes(&self) -> &WorkBiomes<'static> {
        &self.biomes
    }

    pub fn add_level(&mut self, id: &'static str) -> Option<&Arc<RwLock<Level>>> {
        match self.levels.entry(id) {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => {
                Some(v.insert(Level::new(id, Weak::clone(&self.this))))
            }
        }
    }

}
