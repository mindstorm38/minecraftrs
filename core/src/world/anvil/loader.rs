use crate::world::loader::{ChunkLoader, ChunkFactory, ChunkLoadError};
use crate::world::level::{LevelStorage, LevelHeight};
use crate::world::chunk::Chunk;
use super::AnvilManager;


pub struct AnvilChunkLoader {
    manager: AnvilManager
}

impl AnvilChunkLoader {

}

impl ChunkLoader for AnvilChunkLoader {

    fn load_chunk<'env>(&self, factory: &dyn ChunkFactory<'env>, cx: i32, cz: i32) -> Result<Chunk<'env>, ChunkLoadError> {
        todo!()
    }

    fn min_height(&self) -> LevelHeight {
        todo!()
    }

}