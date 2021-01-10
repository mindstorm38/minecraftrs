use crate::block::{BlockRegistry, Block};
use crate::biome::{BiomeRegistry, Biome};
use crate::version::Version;
use crate::rand::jrand;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

pub mod chunk;
pub mod loader;
pub mod gen;

use loader::{ChunkError, ChunkLoader};
use chunk::Chunk;


/// Combine a chunk coordinate pair into 64 bits for hashing.
#[inline]
pub fn combine_chunk_coords(cx: i32, cz: i32) -> u64 {
    cx as u32 as u64 | ((cz as u32 as u64) << 32)
}


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
    loader: Box<dyn ChunkLoader>,
    chunks: HashMap<u64, Chunk>
}

impl World {

    /// Create a new world with a specific seed & version.
    pub fn new(seed: i64, version: Version) -> World {

        let block_registry = BlockRegistry::from(version);
        let biome_registry = BiomeRegistry::from(version);

        let info = Rc::new(WorldInfo {
            version,
            seed,
            block_registry,
            biome_registry
        });

        World {
            loader: gen::for_world(Rc::clone(&info)),
            chunks: HashMap::new(),
            info
        }

    }

    /// Create a new world with a specific version, and a randomly
    /// generated seed.
    pub fn new_seeded(version: Version) -> World {
        Self::new(jrand::gen_seed(), version)
    }

    pub fn get_info(&self) -> &WorldInfo {
        &self.info
    }

    /// Return the list of cached chunks.
    pub fn get_chunks(&self) -> &HashMap<u64, Chunk> {
        &self.chunks
    }

    // PROVIDE CHUNKS //

    /// Provide an existing chunk, if the chunk is not cached the world's
    /// chunk loader is called. If you need a chunk
    pub fn provide_chunk(&mut self, cx: i32, cz: i32) -> Result<&Chunk, ChunkError> {
        match self.chunks.entry(combine_chunk_coords(cx, cz)) {
            Entry::Occupied(o) => Ok(o.into_mut()),
            Entry::Vacant(v) => {
                Ok(v.insert(self.loader.load_chunk(cx, cz)?))
            }
        }
    }

    /// Provide an existing chunk at specific block position, if the chunk is
    /// not cached the world's chunk loader is called.
    pub fn provide_chunk_at(&mut self, x: i32, z: i32) -> Result<&Chunk, ChunkError> {
        self.provide_chunk(x >> 4, z >> 4)
    }

    // CHUNKS //

    /// Get a chunk reference at specific coordinates.
    pub fn get_chunk(&self, cx: i32, cz: i32) -> Option<&Chunk> {
        self.chunks.get(&combine_chunk_coords(cx, cz))
    }

    /// Get a mutable chunk reference at specific coordinates.
    pub fn get_chunk_mut(&mut self, cx: i32, cz: i32) -> Option<&mut Chunk> {
        self.chunks.get_mut(&combine_chunk_coords(cx, cz))
    }

    /// Get a chunk reference at specific blocks coordinates.
    pub fn get_chunk_at(&self, x: i32, z: i32) -> Option<&Chunk> {
        self.get_chunk(x >> 4, z >> 4)
    }

    /// Get a mutable chunk reference at specific blocks coordinates.
    pub fn get_chunk_mut_at(&mut self, x: i32, z: i32) -> Option<&mut Chunk> {
        self.get_chunk_mut(x >> 4, z >> 4)
    }

    // SAFE FUNCTION FOR CHUNKS //

    fn with_chunk_at<'a, F, R>(&'a self, x: i32, y: i32, z: i32, func: F) -> Option<R>
        where F: FnOnce(&'a Chunk, usize, usize, usize) -> Option<R>
    {
        if y < 0 {
            None
        } else {
            let chunk = self.get_chunk_at(x, z)?;
            let y = y as usize;
            if y >= chunk.get_max_height() {
                None
            } else {
                func(chunk, (x & 15) as usize, y, (z & 15) as usize)
            }
        }
    }

    fn with_chunk_mut_at<'a, F>(&'a mut self, x: i32, y: i32, z: i32, func: F)
        where F: FnOnce(&'a mut Chunk, usize, usize, usize)
    {
        if y >= 0 {
            if let Some(chunk) = self.get_chunk_mut_at(x, z) {
                let y = y as usize;
                if y < chunk.get_max_height() {
                    func(chunk, (x & 15) as usize, y, (z & 15) as usize)
                }
            }
        }
    }

    // RAW BLOCKS //

    /// Get a block id at specific position, if the position is invalid, or
    /// the target chunk not loaded, `None` is returned.
    pub fn get_block_id(&self, x: i32, y: i32, z: i32) -> Option<u16> {
        self.with_chunk_at(x, y, z, |c, x, y, z| {
            Some(c.get_block_id(x, y, z))
        })
    }

    // ACTUAL BLOCKS //

    /// Get a block at specific position, if the position is invalid, or
    /// the target chunk not loaded, `None` is returned.
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.info.block_registry.0.get_from_id(self.get_block_id(x, y, z)?)
    }

    /// Set a block at specific position, if the position is invalid nothing happens.
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: Option<&Block>) {
        self.with_chunk_mut_at(x, y, z, |c, x, y, z| {
            c.set_block(x, y, z, block);
        });
    }

    // ACTUAL BIOMES //

    pub fn get_biome_2d(&self, x: i32, z: i32) -> Option<&Biome> {
        self.with_chunk_at(x, 0, z, |c, x, _, z| {
            c.get_biome_2d(x, z)
        })
    }

    pub fn get_biome_3d(&self, x: i32, y: i32, z: i32) -> Option<&Biome> {
        self.with_chunk_at(x, y, z, |c, x, y, z| {
            c.get_biome_3d(x, y, z)
        })
    }

    pub fn set_biome_2d(&mut self, x: i32, z: i32, biome: &Biome) {
        self.with_chunk_mut_at(x, 0, z, |c, x, _, z| {
            c.set_biome_2d(x, z, biome);
        })
    }

    pub fn set_biome_3d(&mut self, x: i32, y: i32, z: i32, biome: &Biome) {
        self.with_chunk_mut_at(x, y, z, |c, x, y, z| {
            c.set_biome_3d(x, y, z, biome);
        })
    }

    // LEGACY BIOMES //

    /*/// Get a biome id at specific position, if the position is invalid, or
    /// the target chunk not loaded, `None` is returned.
    pub fn get_biome_id(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        self.with_chunk_at(x, y, z, |c, x, y, z| {
            Some(c.get_biome_id(x, y, z))
        })
    }

    /// Get a biome at specific position, if the position is invalid, or
    /// the target chunk not loader, `None` is returned.
    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> Option<&Biome> {
        self.info.biome_registry.0.get_from_id(self.get_biome_id(x, y, z)?)
    }

    /// Set a biome at specific position, if the position is invalid nothing happens.
    pub fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: Option<&Biome>) {
        self.with_chunk_mut_at(x, y, z, |c, x, y, z| {
            c.set_biome(x, y, z, biome);
        });
    }*/

}