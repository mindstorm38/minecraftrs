use std::collections::hash_map::Entry;
use std::sync::{Weak, RwLock, Arc};
use std::collections::HashMap;

use super::loader::{ChunkLoader, ChunkFactory, ChunkError, NoChunkLoader};
use super::chunk::Chunk;
use super::World;


/// Main storage for a level, part of a World.
pub struct Level {
    /// The unique ID of this level (among all levels of the world).
    id: &'static str,
    /// Weak counted reference to this structure, this implies that
    /// this structure must be owned by an `Arc<RwLock<_>>`. This is
    /// used internally when building `Chunk`s.
    this: Weak<RwLock<Level>>,
    /// Weak counted reference to the world owning this level.
    world: Weak<RwLock<World>>,
    /// Chunk storage, stored in another field to allow the loader, and
    /// the storage to be mutated concurrently.
    storage: LevelStorage,
    /// The chunk loader used to load uncached chunks.
    loader: Box<dyn ChunkLoader>,
}

impl Level {

    pub fn new(id: &'static str, world: Weak<RwLock<World>>) -> Arc<RwLock<Level>> {

        world.upgrade().expect("The given world weak reference must be valid at construction.");

        let ret = Arc::new(RwLock::new(Level {
            id,
            this: Weak::new(),
            world,
            storage: LevelStorage {
                chunks: HashMap::new()
            },
            loader: Box::new(NoChunkLoader)
        }));

        ret.write().unwrap().this = Arc::downgrade(&ret);
        ret

    }

    pub fn get_id(&self) -> &'static str {
        self.id
    }

    /// Return a strong counted reference to the `World` owning this level.
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
    }

    // PROVIDE CHUNKS //

    /// Provide an existing chunk, if the chunk is not cached the world's
    /// chunk loader is called. If you need a chunk
    pub fn provide_chunk(&mut self, cx: i32, cz: i32) -> Result<&Chunk, ChunkError> {

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
    pub fn provide_chunk_at(&mut self, x: i32, z: i32) -> Result<&Chunk, ChunkError> {
        self.provide_chunk(x >> 4, z >> 4)
    }

    /// Internal function used to ensure a chunk in the `chunks` HashMap, or return an error
    /// if the loading fails.
    fn ensure_chunk(&mut self, cx: i32, cz: i32) -> Result<(), ChunkError> {
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

    fn expect_chunk(&self, cx: i32, cz: i32) -> &Chunk {
        match self.storage.get_chunk(cx, cz) {
            None => panic!("Unexpected unloaded chunk {}/{}", cx, cz),
            Some(chunk) => chunk
        }
    }

    pub fn get_storage(&self) -> &LevelStorage {
        &self.storage
    }

    pub fn get_storage_mut(&mut self) -> &mut LevelStorage {
        &mut self.storage
    }

}


/// Internal level storage.
pub struct LevelStorage {
    /// Storing all cached chunks.
    chunks: HashMap<u64, Chunk>,
}


impl LevelStorage {

    // CHUNKS //

    /// Return true if a chunk is loaded at a specific position.
    pub fn is_chunk_loaded(&self, cx: i32, cz: i32) -> bool {
        self.chunks.contains_key(&combine_chunk_coords(cx, cz))
    }

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

}


struct LevelChunkFactory<'a> {
    weak_level: &'a Weak<RwLock<Level>>,
    cx: i32,
    cz: i32
}

impl ChunkFactory for LevelChunkFactory<'_> {

    fn build(&self, sub_chunks_count: u8) -> Chunk {
        Chunk::new(Weak::clone(self.weak_level), self.cx, self.cz, sub_chunks_count)
    }

}


/// Combine a chunk coordinate pair into 64 bits for hashing.
#[inline]
pub fn combine_chunk_coords(cx: i32, cz: i32) -> u64 {
    cx as u32 as u64 | ((cz as u32 as u64) << 32)
}
