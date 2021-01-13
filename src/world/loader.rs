use super::chunk::Chunk;
use super::ChunkMap;


/// Common chunk error enum
#[derive(Debug)]
pub enum ChunkError {
    IllegalPosition(i32, i32),
    CannotPopulate
}


/// Chunk loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait ChunkLoader {
    fn load_chunk(&self, cx: i32, cz: i32) -> Result<Chunk, ChunkError>;
    fn populate_chunk(&self, world: &mut ChunkMap, cx: i32, cz: i32) -> Result<(), ChunkError>;
}
