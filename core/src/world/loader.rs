use super::level::{LevelStorage, LevelHeight};
use super::chunk::Chunk;


/// Common chunk error enum
#[derive(Debug)]
pub enum ChunkLoadError {
    IllegalPosition(i32, i32)
}


/// This factory trait is implemented by the caller of `ChunkLoader::load_chunk`
/// and must construct the chunk at the position given as parameters (`cx`, `cz`)
/// for `ChunkLoader::load_chunk`.
pub trait ChunkFactory<'env> {

    fn build(&self) -> Chunk<'env>;

    fn build_populated(&self) -> Chunk<'env> {
        let mut chunk = self.build();
        chunk.set_populated(true);
        chunk
    }

}


/// Chunk loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait ChunkLoader {

    /// Load the chunk at specified position, generators must generate.
    fn load_chunk<'env>(&self, factory: &dyn ChunkFactory<'env>, cx: i32, cz: i32) -> Result<Chunk<'env>, ChunkLoadError>;

    /// Because the chunk population mechanism involve multiples chunks checks,
    /// the chunk population must be generated separately from the `load_chunk`.
    ///
    /// Implementations that does not support populating should load chunk and
    /// set the populated flag to true and should panic with `unimplemented!`
    /// macro if this function is called.
    fn populate_chunk(&self, world: &mut LevelStorage, cx: i32, cz: i32);

    fn min_height(&self) -> LevelHeight;

}


/// Dummy chunk loader which do nothing.
pub struct NoChunkLoader;

impl ChunkLoader for NoChunkLoader {

    fn load_chunk<'env>(&self, _: &dyn ChunkFactory<'env>, cx: i32, cz: i32) -> Result<Chunk<'env>, ChunkLoadError> {
        Err(ChunkLoadError::IllegalPosition(cx, cz))
    }

    fn populate_chunk(&self, _: &mut LevelStorage, _: i32, _: i32) {
        unimplemented!("NoChunkLoader doesn't provide chunks.");
    }

    fn min_height(&self) -> LevelHeight {
        LevelHeight {
            min: 0,
            max: 0
        }
    }

}
