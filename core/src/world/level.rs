use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;

use crate::block::GlobalBlocks;
use crate::biome::GlobalBiomes;
use crate::debug;

use super::source::{LevelSource, LevelSourceBuilder, ChunkBuilder};
use super::chunk::{Chunk, ChunkHeight};


/// A structure that contains the static environment of a World, this can be used for multiple
/// `Level`s through an `Arc<LevelEnv>`.
pub struct LevelEnv {
    /// Actual blocks register.
    pub blocks: GlobalBlocks<'static>,
    /// Actual biomes register.
    pub biomes: GlobalBiomes<'static>
}

impl LevelEnv {

    pub fn new(
        blocks: GlobalBlocks<'static>,
        biomes: GlobalBiomes<'static>
    ) -> Self {
        LevelEnv {
            blocks,
            biomes
        }
    }

    #[cfg(all(feature = "vanilla_blocks", feature = "vanilla_biomes"))]
    pub fn new_vanilla() -> Result<Self, ()> {
        Ok(Self::new(GlobalBlocks::new_vanilla()?, GlobalBiomes::new_vanilla()?))
    }

}


/// Main storage for a level, part of a World.
pub struct Level<S: LevelSource> {
    /// The unique ID of this level (among all levels of the world).
    id: String,
    /// The global environment used by this level, this environment should not be mutated afterward.
    /// It contains the global blocks and biomes palettes, it also contains
    env: Arc<LevelEnv>,
    /// The level loader used to load uncached chunks either from a generator or from an anvil file
    /// system loader.
    source: S,
    /// The configured height of this level.
    height: ChunkHeight,
    /// Chunk storage, stored in another field to allow the loader, and
    /// the storage to be mutated concurrently.
    storage: LevelStorage,
}

impl<S: LevelSource> Level<S> {

    pub fn new<B>(id: String, env: Arc<LevelEnv>, source: B) -> Self
    where
        B: LevelSourceBuilder<S>
    {

        assert_ne!(env.blocks.states_count(), 0, "The given environment has no state, a level requires at least one block state.");
        assert_ne!(env.biomes.biomes_count(), 0, "The given environment has no biome, a level requires at least one biome.");

        let height = source.get_height();
        let builder = ChunkBuilder {
            env: Arc::clone(&env),
            height
        };

        Level {
            id,
            env,
            height,
            source: source.build(builder),
            storage: LevelStorage {
                chunks: HashMap::new()
            }
        }

    }

    /// Return the unique ID (unique in the owning world).
    pub fn get_id(&self) -> &String {
        &self.id
    }

    /// Return the level environment used by this level.
    pub fn get_env(&self) -> Arc<LevelEnv> {
        Arc::clone(&self.env)
    }

    /// Return the minimum and maximum chunks position allowed in this world.
    /// The limits can -128 to 127, it is more than enough.
    pub fn get_height(&self) -> ChunkHeight {
        self.height
    }

    pub fn request_chunk(&mut self, cx: i32, cz: i32) -> bool {
        debug!("Request chunk load at {}/{}", cx, cz);
        matches!(self.source.request_chunk_load(cx, cz), Ok(_))
    }

    pub fn load_chunks(&mut self) {
        while let Some(((cx, cz), res)) = self.source.poll_chunk() {
            match res {
                Ok(chunk) => {
                    debug!("Loaded chunk at {}/{}", cx, cz);
                    self.storage.insert_chunk(chunk)
                },
                Err(err) => {
                    // IDE shows an error for 'Display' not being implemented, but we use the
                    // crate 'thiserror' to implement it through a custom derive.
                    debug!("Failed to load chunk at {}/{}: {}", cx, cz, err);
                }
            }
        }
    }

    pub fn get_storage(&self) -> &LevelStorage {
        &self.storage
    }

    pub fn mut_storage(&mut self) -> &mut LevelStorage {
        &mut self.storage
    }

}


/// Internal level storage.
pub struct LevelStorage {
    /// Storing all cached chunks.
    chunks: HashMap<(i32, i32), Arc<RwLock<Chunk>>>,
}


impl LevelStorage {

    // CHUNKS //

    /// Insert a chunk at a specific position.
    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.get_position(), Arc::new(RwLock::new(chunk)));
    }

    /// Return true if a chunk is loaded at a specific position.
    pub fn is_chunk_loaded(&self, cx: i32, cz: i32) -> bool {
        self.chunks.contains_key(&(cx, cz))
    }

    /// Get a chunk reference at specific coordinates.
    pub fn get_chunk(&self, cx: i32, cz: i32) -> Option<RwLockReadGuard<Chunk>> {
        self.chunks.get(&(cx, cz)).map(|arc| arc.read().unwrap())
    }

    /// Get a mutable chunk reference at specific coordinates.
    pub fn mut_chunk(&self, cx: i32, cz: i32) -> Option<RwLockWriteGuard<Chunk>> {
        self.chunks.get(&(cx, cz)).map(|arc| arc.write().unwrap())
    }

    /// Get a chunk reference at specific blocks coordinates.
    pub fn get_chunk_at(&self, x: i32, z: i32) -> Option<RwLockReadGuard<Chunk>> {
        self.get_chunk(x >> 4, z >> 4)
    }

    /// Get a mutable chunk reference at specific blocks coordinates.
    pub fn mut_chunk_at(&self, x: i32, z: i32) -> Option<RwLockWriteGuard<Chunk>> {
        self.mut_chunk(x >> 4, z >> 4)
    }

}


/// Combine a chunk coordinate pair into 64 bits for hashing.
#[inline]
pub fn combine_chunk_coords(cx: i32, cz: i32) -> u64 {
    cx as u32 as u64 | ((cz as u32 as u64) << 32)
}
