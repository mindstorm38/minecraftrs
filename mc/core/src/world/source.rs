use std::error::Error;
use std::sync::Arc;

use thiserror::Error;

use super::chunk::{Chunk, ChunkHeight};
use crate::world::level::LevelEnv;


/// Common level source error.
#[derive(Error, Debug)]
pub enum LevelSourceError {
    #[error("The required chunk can't be loaded because its position is illegal.")]
    IllegalChunkPosition,
    #[error("Returned by `LevelLoader` methods that does not support the operation '{0}'.")]
    UnsupportedOperation(&'static str),
    #[error("{0}")]
    Custom(Box<dyn Error + Send>)
}

impl LevelSourceError {
    pub fn new_custom(err: impl Error + Send + 'static) -> Self {
        Self::Custom(Box::new(err))
    }
}

/// A type alias for a result with error of type `LevelSourceError`.
pub type LevelSourceResult<T> = Result<T, LevelSourceError>;

/// Type alias for the result when polling a chunk.
pub type LevelSourcePollResult = ((i32, i32), LevelSourceResult<Chunk>);
// TODO: Change result Ok type to a 'ProtoChunk'. This will be useful to
//       actually decode entities in sync.


/// Level loader trait to implement for each different loader such as
/// disk or generator loaders, combine these two types of loaders to
/// save and load change in a chunk to avoid generating twice.
pub trait LevelSource {

    /// Request loading of the chunk at the given position. If you return `Ok(())` here you SHOULD
    /// produce some `LevelSourcePollResult` in `poll_chunk` method, even if it's an error.
    #[allow(unused)]
    fn request_chunk_load(&mut self, cx: i32, cz: i32) -> LevelSourceResult<()> {
        Err(LevelSourceError::UnsupportedOperation("request_chunk_load"))
    }

    /// Poll the next loaded chunk that is ready to be inserted into the level's chunk storage.
    /// Every requested load chunk `request_chunk_load` method that returned `Ok(())` should
    /// return some poll result here, even if it's an error.
    #[allow(unused)]
    fn poll_chunk(&mut self) -> Option<LevelSourcePollResult> {
        None
    }

}


/// This trait is used to build a `LevelSource` object. This intermediate
/// step is useful in case of anvil level loader, this allows the builder
/// to guess for the right chunk height in the level metadata.
pub trait LevelSourceBuilder<S: LevelSource> {

    /// This method should return the level height to be used by the level.
    fn get_height(&self) -> ChunkHeight {
        ChunkHeight { min: 0, max: 15 }
    }

    /// Build the level source, the given chunk builder must be used in the
    /// source to build new chunks.
    fn build(self, chunk_builder: ChunkBuilder) -> S;

}


/// This structure is constructed by levels and passed to `LevelSourceBuilder`
/// when building the `LevelSource`, this level source will need to use this
/// builder for new chunks.
#[derive(Clone)]
pub struct ChunkBuilder {
    pub(super) env: Arc<LevelEnv>,
    pub(super) height: ChunkHeight
}

impl ChunkBuilder {

    /// Actually build a new chunk that is compatible with the level that
    /// produced this chunk builder.
    pub fn build(&self, cx: i32, cz: i32) -> Chunk {
        Chunk::new(Arc::clone(&self.env), self.height, cx, cz)
    }

}


/// Dummy chunk loader which do nothing.
pub struct NullLevelSource;
impl LevelSource for NullLevelSource {}



pub struct