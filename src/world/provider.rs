use crate::world::chunk::Chunk;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


/// Common chunk error enum
#[derive(Debug)]
pub enum ChunkError {
    IllegalPosition(i32, i32)
}


/// Combine a chunk coordinate pair into 64 bits for hashing.
#[inline]
pub const fn combine_chunk_coords(cx: i32, cz: i32) -> u64 {
    cx as u64 | ((cz as u64) << 32)
}


/// Chunk loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait ChunkLoader {
    fn load_chunk(&self, cx: i32, cz: i32) -> Result<Chunk, ChunkError>;
}


/// Chunk cacher, caching chunks with efficient hash map. If the chunk
/// is not loaded, the chunk loader is called.
pub struct ChunkCacher {
    loader: Box<dyn ChunkLoader>,
    chunks: HashMap<u64, Chunk>
}

impl ChunkCacher {

    pub fn new(loader: Box<dyn ChunkLoader>) -> ChunkCacher {
        ChunkCacher {
            loader,
            chunks: HashMap::new()
        }
    }

    pub fn provide_chunk(&mut self, cx: i32, cz: i32) -> Result<&mut Chunk, ChunkError> {
        match self.chunks.entry(combine_chunk_coords(cx, cz)) {
            Entry::Occupied(o) => Ok(o.into_mut()),
            Entry::Vacant(v) => {
                Ok(v.insert(self.loader.load_chunk(cx, cz)?))
            }
        }
    }

    pub fn get_chunks(&self) -> &HashMap<u64, Chunk> {
        &self.chunks
    }

}