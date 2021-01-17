use super::chunk::Chunk;
use super::WorldAccess;


/// Common chunk error enum
#[derive(Debug)]
pub enum ChunkError {
    IllegalPosition(i32, i32)
}


/// Chunk loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait ChunkLoader {

    /// Load the chunk at specified position, generators must generate.
    fn load_chunk(&self, cx: i32, cz: i32) -> Result<Chunk, ChunkError>;

    /// Because the chunk population mechanism involve multiples chunks checks,
    /// the chunk population must be generated separately from the `load_chunk`.
    ///
    /// Implementations that does not support populating should load chunk and
    /// set the populated flag to true and should panic with `unimplemented!`
    /// macro if this function is called.
    fn populate_chunk(&self, world: &mut WorldAccess, cx: i32, cz: i32);

}
