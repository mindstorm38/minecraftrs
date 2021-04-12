use super::level::LevelStorage;
use super::chunk::Chunk;


/// Common chunk error enum
#[derive(Debug)]
pub enum ChunkError {
    IllegalPosition(i32, i32)
}


/// This factory trait is implemented by the caller of `ChunkLoader::load_chunk`
/// and must construct the chunk at the position given as parameters (`cx`, `cz`)
/// for `ChunkLoader::load_chunk`.
pub trait ChunkFactory {

    fn build(&self, sub_chunks_count: u8) -> Chunk;

    fn build_populated(&self, sub_chunks_count: u8) -> Chunk {
        let mut chunk = self.build(sub_chunks_count);
        chunk.set_populated(true);
        chunk
    }

}


/// Chunk loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait ChunkLoader {

    /// Load the chunk at specified position, generators must generate.
    fn load_chunk(&self, factory: &dyn ChunkFactory, cx: i32, cz: i32) -> Result<Chunk, ChunkError>;

    /// Because the chunk population mechanism involve multiples chunks checks,
    /// the chunk population must be generated separately from the `load_chunk`.
    ///
    /// Implementations that does not support populating should load chunk and
    /// set the populated flag to true and should panic with `unimplemented!`
    /// macro if this function is called.
    fn populate_chunk(&self, world: &mut LevelStorage, cx: i32, cz: i32);

}


/// Dummy chunk loader which do nothing.
pub struct NoChunkLoader;

impl ChunkLoader for NoChunkLoader {

    fn load_chunk(&self, _: &dyn ChunkFactory, cx: i32, cz: i32) -> Result<Chunk, ChunkError> {
        Err(ChunkError::IllegalPosition(cx, cz))
    }

    fn populate_chunk(&self, _: &mut LevelStorage, _: i32, _: i32) {
        unimplemented!("NoChunkLoader doesn't provide chunks.");
    }

}
