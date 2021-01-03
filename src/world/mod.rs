use crate::block::registry::BlockRegistry;
use crate::biome::registry::BiomeRegistry;
use crate::version::Version;
use crate::rand::jrand;
use std::rc::Rc;

pub mod chunk;
pub mod provider;
pub mod gen;

use provider::{ChunkCacher, ChunkError};
use chunk::Chunk;


/// World info are used to be shared with the chunk loader.
pub struct WorldInfo {
    pub version: Version,
    pub seed: i64,
    pub block_registry: BlockRegistry,
    pub biome_registry: BiomeRegistry
}

/// A world for a specific version with specific registries and chunk loaders.
pub struct World {
    info: Rc<WorldInfo>,
    chunk_cacher: ChunkCacher
}

impl World {

    pub fn new(seed: i64, version: Version) -> World {

        let block_registry = BlockRegistry::from(version);
        let biome_registry = BiomeRegistry::from(version);

        let info = Rc::new(WorldInfo {
            version,
            seed,
            block_registry,
            biome_registry
        });

        let chunk_loader = gen::for_world(Rc::clone(&info));

        World {
            info,
            chunk_cacher: ChunkCacher::new(chunk_loader)
        }

    }

    pub fn new_seeded(version: Version) -> World {
        Self::new(jrand::gen_seed(), version)
    }

    pub fn get_info(&self) -> &WorldInfo {
        &self.info
    }

    #[inline]
    pub fn get_chunk_at(&mut self, cx: i32, cz: i32) -> Result<&mut Chunk, ChunkError> {
        self.chunk_cacher.provide_chunk(cx, cz)
    }

    #[inline]
    pub fn get_chunk_at_block(&mut self, x: i32, z: i32) -> Result<&mut Chunk, ChunkError> {
        self.get_chunk_at(x >> 4, z >> 4)
    }

}