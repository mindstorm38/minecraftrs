//! This module provides traits that allows abstraction of real level's concepts, called views.
//! Since `Level` and `Chunk` behaviors are not modifiable .

use std::sync::Arc;

use mc_core::world::chunk::{Chunk, ChunkResult};
use mc_core::world::source::ProtoChunk;
use mc_core::heightmap::HeightmapType;
use mc_core::world::level::LevelEnv;
use mc_core::block::BlockState;
use mc_core::biome::Biome;


/// A local level view used to generate feature in an partial level view.
pub trait LevelView {

    /// Get a reference to the shared level environment.
    fn get_env(&self) -> &Arc<LevelEnv>;

    fn get_chunk(&self, cx: i32, cz: i32) -> Option<&Chunk>;
    fn get_chunk_mut(&mut self, cx: i32, cz: i32) -> Option<&mut Chunk>;

    #[inline]
    fn get_chunk_at(&self, x: i32, z: i32) -> Option<&Chunk> {
        self.get_chunk(x >> 4, z >> 4)
    }

    #[inline]
    fn get_chunk_at_mut(&mut self, x: i32, z: i32) -> Option<&mut Chunk> {
        self.get_chunk_mut(x >> 4, z >> 4)
    }

    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()>;
    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState>;

    fn get_biome_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static Biome>;

    fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32>;

}


/// A trait to implement customized proto chunks that can behave differently from a normal
/// proto chunk. This view can be converted back to its inner value.
///
/// TODO: It might be useful to have method with local chunk coordinates (x/z: u8).
pub trait ProtoChunkView {

    fn into_inner(self) -> ProtoChunk;

    fn as_chunk_ref(&self) -> &Chunk;
    fn as_chunk_mut(&mut self) -> &mut Chunk;

    fn get_position(&self) -> (i32, i32);

    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState>;
    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()>;

    fn get_biome_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static Biome>;

    fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32>;

}

impl ProtoChunkView for ProtoChunk {

    fn into_inner(self) -> ProtoChunk {
        self
    }

    #[inline]
    fn as_chunk_ref(&self) -> &Chunk {
        &**self
    }

    #[inline]
    fn as_chunk_mut(&mut self) -> &mut Chunk {
        &mut **self
    }

    #[inline]
    fn get_position(&self) -> (i32, i32) {
        Chunk::get_position(self.as_chunk_ref())
    }

    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState> {
        Chunk::get_block_at(self.as_chunk_ref(), x, y, z)
    }

    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()> {
        Chunk::set_block_at(self.as_chunk_mut(), x, y, z, state)
    }

    fn get_biome_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static Biome> {
        Chunk::get_biome_at(self.as_chunk_ref(), x, y, z)
    }

    fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32> {
        Chunk::get_heightmap_column_at(self.as_chunk_ref(), heightmap_type, x, z)
    }

}
