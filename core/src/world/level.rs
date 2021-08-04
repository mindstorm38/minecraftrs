use std::collections::hash_map::Entry;
use std::sync::{Weak, RwLock, Arc};
use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::block::WorkBlocks;
use crate::biome::WorkBiomes;

use super::loader::{ChunkLoader, ChunkFactory, ChunkLoadError, NoChunkLoader};
use super::chunk::Chunk;


/// A structure that contains the static environment of a World, this can be used for multiple
/// Levels and it's used to decide was
pub struct LevelEnv {
    /// Actual blocks register.
    blocks: WorkBlocks<'static>,
    /// Actual biomes register.
    biomes: WorkBiomes<'static>,
}

impl LevelEnv {

    pub fn new(blocks: WorkBlocks<'static>, biomes: WorkBiomes<'static>) -> Self {
        LevelEnv { blocks, biomes }
    }

    #[cfg(all(feature = "vanilla_blocks", feature = "vanilla_biomes"))]
    pub fn new_vanilla() -> Result<Self, ()> {
        Ok(Self::new(WorkBlocks::new_vanilla()?, WorkBiomes::new_vanilla()?))
    }

    pub fn get_blocks(&self) -> &WorkBlocks<'static> {
        &self.blocks
    }

    pub fn get_biomes(&self) -> &WorkBiomes<'static> {
        &self.biomes
    }

}


/// This structure is used to represent the physical limits of a level.
#[derive(Debug, Clone, Copy)]
pub struct LevelHeight {
    pub min: i8,
    pub max: i8
}

impl LevelHeight {

    #[inline]
    pub fn includes(self, cy: i8) -> bool {
        return self.min <= cy && cy <= self.max;
    }

}

impl IntoIterator for LevelHeight {

    type Item = i8;
    type IntoIter = RangeInclusive<i8>;

    fn into_iter(self) -> Self::IntoIter {
        self.min..=self.max
    }

}


/// This structure must be used to configure a `Level`.
pub struct LevelBuilder {
    id: &'static str,
    height: LevelHeight,
    loader: Box<dyn ChunkLoader>
}

impl LevelBuilder {

    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            height: LevelHeight { min: 0, max: 0 },
            loader: Box::new(NoChunkLoader)
        }
    }

    pub fn get_id(&self) -> &'static str {
        self.id
    }

    /// Use a specific `ChunkLoader` for this world. If this current height
    /// limits are shorter than the loader requirements, the loader limits
    /// are used.
    ///
    /// # Panics
    ///
    /// This method panics is the loader return an invalid height range (min > max).
    pub fn with_loader(mut self, loader: impl ChunkLoader + 'static) -> Self {

        let (min, max) = loader.min_height();
        debug_assert!(min <= max, "The loader height has a minimum higher than the maximum.");

        self.loader = Box::new(loader);

        if min < self.height.min {
            self.height.min = min;
        }

        if max > self.height.max {
            self.height.max = max;
        }

        self

    }

    /// Set the height limits for the level, if the given limits are not
    /// allowed by the current `ChunkLoader`, the limit is not saved.
    ///
    /// The limits are expressed in vertical chunks coordinates.
    ///
    /// The limit of the height are the limits of 8 bits integers (-128 to 127 included,
    /// so 256 maximum chunks in the height, 4096 blocks).
    pub fn with_height(mut self, min: i8, max: i8) -> Self {

        debug_assert!(min <= max, "The given minimum higher than the maximum.");

        let (ldr_min, ldr_max) = self.loader.min_height();

        if min < ldr_min {
            self.height.min = min;
        }

        if max > ldr_max {
            self.height.max = max;
        }

        self

    }

    /// Delegate call to `Level::new`.
    #[inline]
    pub fn build(self, env: &LevelEnv) -> Arc<RwLock<Level>> {
        Level::new(self, env)
    }

}


/// Main storage for a level, part of a World.
pub struct Level<'env> {
    env: &'env LevelEnv,
    /// The unique ID of this level (among all levels of the world).
    id: &'static str,
    /// Weak counted reference to this structure, this implies that
    /// this structure must be owned by an `Arc<RwLock<_>>`. This is
    /// used internally when building `Chunk`s.
    this: Weak<RwLock<Level<'env>>>,
    /// The minimum and maximum chunks coordinates allowed.
    height: LevelHeight,
    /// Chunk storage, stored in another field to allow the loader, and
    /// the storage to be mutated concurrently.
    storage: LevelStorage<'env>,
    /// The chunk loader used to load uncached chunks.
    loader: Box<dyn ChunkLoader>,
}

impl<'env> Level<'env> {

    fn new(builder: LevelBuilder, env: &'env LevelEnv) -> Arc<RwLock<Level<'env>>> {

        assert_ne!(env.blocks.states_count(), 0, "The given environment has no states, a level requires at least one block state with save ID 0");
        assert_ne!(env.biomes.biomes_count(), 0, "The given environment has no biomes, a level requires at least one biome with save ID 0");

        let ret = Arc::new(RwLock::new(Level {
            env,
            id: builder.id,
            this: Weak::new(),
            height: builder.height,
            storage: LevelStorage {
                chunks: HashMap::new()
            },
            loader: builder.loader
        }));

        ret.write().unwrap().this = Arc::downgrade(&ret);
        ret

    }

    /// Return the level environment used by this level.
    pub fn get_env(&self) -> &'env LevelEnv {
        self.env
    }

    /// Return the unique ID (unique in the owning world).
    pub fn get_id(&self) -> &'static str {
        self.id
    }

    /// Return the minimum and maximum chunks position allowed in this world.
    /// The limits can -128 to 127, it is more than enough.
    pub fn get_height(&self) -> LevelHeight {
        self.height
    }

    /*/// Return a strong counted reference to the `World` owning this level.
    ///
    /// # Panics
    ///
    /// This method panic if this level is no longer owned (should not happen).
    pub fn get_world(&self) -> Arc<RwLock<World>> {
        self.world.upgrade().expect("This level is no longer owned by its world.")
    }

    /// Return a weak counted reference to the `World` owning this level.
    pub fn get_weak_world(&self) -> Weak<RwLock<World>> {
        Weak::clone(&self.world)
    }*/

    // PROVIDE CHUNKS //

    /// Provide an existing chunk, if the chunk is not cached the world's
    /// chunk loader is called. If you need a chunk
    pub fn provide_chunk(&mut self, cx: i32, cz: i32) -> Result<&Chunk<'env>, ChunkLoadError> {

        self.ensure_chunk(cx, cz)?;

        if !self.expect_chunk(cx, cz).is_populated() &&
            self.storage.is_chunk_loaded(cx + 1, cz) &&
            self.storage.is_chunk_loaded(cx, cz + 1) &&
            self.storage.is_chunk_loaded(cx + 1, cz + 1) {

            self.loader.populate_chunk(&mut self.storage, cx, cz);

        }

        if let Some(chunk) = self.storage.get_chunk(cx - 1, cz) {
            if !chunk.is_populated() && self.storage.is_chunk_loaded(cx - 1, cz + 1) && self.storage.is_chunk_loaded(cx, cz + 1) {
                self.loader.populate_chunk(&mut self.storage, cx - 1, cz);
            }
        }

        if let Some(chunk) = self.storage.get_chunk(cx, cz - 1) {
            if !chunk.is_populated() && self.storage.is_chunk_loaded(cx + 1, cz - 1) && self.storage.is_chunk_loaded(cx + 1, cz) {
                self.loader.populate_chunk(&mut self.storage, cx, cz - 1);
            }
        }

        if let Some(chunk) = self.storage.get_chunk(cx - 1, cz - 1) {
            if !chunk.is_populated() && self.storage.is_chunk_loaded(cx - 1, cz) && self.storage.is_chunk_loaded(cx, cz - 1) {
                self.loader.populate_chunk(&mut self.storage, cx - 1, cz - 1);
            }
        }

        Ok(self.expect_chunk(cx, cz))

    }

    /// Provide an existing chunk at specific block position, if the chunk is
    /// not cached the world's chunk loader is called.
    pub fn provide_chunk_at(&mut self, x: i32, z: i32) -> Result<&Chunk<'env>, ChunkLoadError> {
        self.provide_chunk(x >> 4, z >> 4)
    }

    /// Internal function used to ensure a chunk in the `chunks` HashMap, or return an error
    /// if the loading fails.
    fn ensure_chunk(&mut self, cx: i32, cz: i32) -> Result<(), ChunkLoadError> {
        match self.storage.chunks.entry(combine_chunk_coords(cx, cz)) {
            Entry::Occupied(_) => Ok(()),
            Entry::Vacant(v) => {

                let factory = LevelChunkFactory {
                    weak_level: &self.this,
                    cx,
                    cz
                };

                v.insert(self.loader.load_chunk(&factory, cx, cz)?);
                Ok(())

            }
        }
    }

    fn expect_chunk(&self, cx: i32, cz: i32) -> &Chunk<'env> {
        match self.storage.get_chunk(cx, cz) {
            None => panic!("Unexpected unloaded chunk {}/{}", cx, cz),
            Some(chunk) => chunk
        }
    }

    pub fn get_storage(&self) -> &LevelStorage<'env> {
        &self.storage
    }

    pub fn mut_storage(&mut self) -> &mut LevelStorage<'env> {
        &mut self.storage
    }

}


/// Internal level storage.
pub struct LevelStorage<'env> {
    /// Storing all cached chunks.
    chunks: HashMap<u64, Chunk<'env>>,
}


impl<'env> LevelStorage<'env> {

    // CHUNKS //

    /// Return true if a chunk is loaded at a specific position.
    pub fn is_chunk_loaded(&self, cx: i32, cz: i32) -> bool {
        self.chunks.contains_key(&combine_chunk_coords(cx, cz))
    }

    /// Get a chunk reference at specific coordinates.
    pub fn get_chunk(&self, cx: i32, cz: i32) -> Option<&Chunk<'env>> {
        self.chunks.get(&combine_chunk_coords(cx, cz))
    }

    /// Get a mutable chunk reference at specific coordinates.
    pub fn mut_chunk(&mut self, cx: i32, cz: i32) -> Option<&mut Chunk<'env>> {
        self.chunks.get_mut(&combine_chunk_coords(cx, cz))
    }

    /// Get a chunk reference at specific blocks coordinates.
    pub fn get_chunk_at(&self, x: i32, z: i32) -> Option<&Chunk<'env>> {
        self.get_chunk(x >> 4, z >> 4)
    }

    /// Get a mutable chunk reference at specific blocks coordinates.
    pub fn mut_chunk_at(&mut self, x: i32, z: i32) -> Option<&mut Chunk<'env>> {
        self.mut_chunk(x >> 4, z >> 4)
    }

}


struct LevelChunkFactory<'a, 'env> {
    weak_level: &'a Weak<RwLock<Level<'env>>>,
    cx: i32,
    cz: i32
}

impl<'env> ChunkFactory<'env> for LevelChunkFactory<'_, 'env> {

    fn build(&self) -> Chunk<'env> {
        Chunk::new(Weak::clone(self.weak_level), self.cx, self.cz)
    }

}


/// Combine a chunk coordinate pair into 64 bits for hashing.
#[inline]
pub fn combine_chunk_coords(cx: i32, cz: i32) -> u64 {
    cx as u32 as u64 | ((cz as u32 as u64) << 32)
}
