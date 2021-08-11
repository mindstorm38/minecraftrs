use crate::world::loader::{ChunkLoader, ChunkFactory, ChunkLoadError};
use crate::world::level::{LevelHeight};
use crate::world::chunk::Chunk;
use super::AnvilManager;


pub struct AnvilChunkLoader {
    manager: AnvilManager
}

impl ChunkLoader for AnvilChunkLoader {

    fn load_chunk<'env>(&self, factory: &dyn ChunkFactory<'env>, cx: i32, cz: i32) -> Result<Chunk<'env>, ChunkLoadError> {

        /*let region_arc = self.manager.ensure_region(cx, cz).unwrap();
        let mut region = region_arc.lock().unwrap();
        let mut reader = region.get_chunk_reader(cx, cz).unwrap();
        let tag = read_compound_tag(&mut reader).unwrap();*/

        todo!()

    }

    fn min_height(&self) -> LevelHeight {
        todo!()
    }

}
