use crate::res::Registrable;
use crate::block::{Block, BlockRegistry};
use crate::biome::{Biome, BiomeRegistry};
use super::WorldInfo;
use std::rc::Rc;
use std::borrow::Borrow;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const DATA_SIZE: usize = SIZE * SIZE * SIZE;


/// A vertical chunk, 16x16 blocks.
pub struct Chunk {
    world_info: Rc<WorldInfo>,
    cx: i32,
    cz: i32,
    sub_chunks: Vec<SubChunk>
}

impl Chunk {

    pub fn new(world_info: Rc<WorldInfo>, cx: i32, cz: i32, sub_chunks_count: u8) -> Self {
        Chunk {
            cx,
            cz,
            sub_chunks: (0..sub_chunks_count).map(|_| SubChunk::new(Rc::clone(&world_info))).collect(),
            world_info
        }
    }

    /// Get the chunk position `(x, z)`.
    pub fn get_position(&self) -> (i32, i32) {
        (self.cx, self.cz)
    }

    /// Get a sub chunk reference at a specified index.
    pub fn get_sub_chunk(&self, cy: usize) -> &SubChunk {
        &self.sub_chunks[cy]
    }

    /// Get a sub chunk mutable reference at a specified index.
    pub fn get_sub_chunk_mut(&mut self, cy: usize) -> &mut SubChunk {
        &mut self.sub_chunks[cy]
    }

    /// Return the number of sub chunks in the height of this chunk.
    pub fn get_sub_chunks_count(&self) -> usize {
        self.sub_chunks.len()
    }

    /// Get the max number of blocks in the height of this chunk.
    pub fn get_max_height(&self) -> usize {
        self.get_sub_chunks_count() << 4
    }

    pub fn get_sub_chunks(&self) -> &Vec<SubChunk> {
        &self.sub_chunks
    }

    // RAW BLOCKS //

    /// Get the block id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn get_block_id(&self, x: usize, y: usize, z: usize) -> u16 {
        self.get_sub_chunk(y >> 4).get_block_id(x, y & 15, z)
    }

    /// Set the block id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn set_block_id(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        self.get_sub_chunk_mut(y >> 4).set_block_id(x, y & 15, z, block_id);
    }

    // ACTUAL BLOCKS //

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        self.get_sub_chunk(y >> 4).get_block(x, y & 15, z)
    }

    /// Set the block at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Option<&Block>) {
        self.get_sub_chunk_mut(y >> 4).set_block(x, y & 15, z, block);
    }

    // RAW BIOMES //
    // TODO

    // ACTUAL BIOMES //
    // TODO

    // LEGACY BIOMES //

    /// Get the biome id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    #[deprecated]
    pub fn get_biome_id(&self, x: usize, y: usize, z: usize) -> u8 {
        self.get_sub_chunk(y >> 4).get_biome_id(x, y & 15, z)
    }

    /// Set the biome id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    #[deprecated]
    pub fn set_biome_id(&mut self, x: usize, y: usize, z: usize, biome_id: u8) {
        self.get_sub_chunk_mut(y >> 4).set_biome_id(x, y & 15, z, biome_id);
    }

    /// Set the biome at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    #[deprecated]
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: Option<&Biome>) {
        self.get_sub_chunk_mut(y >> 4).set_biome(x, y & 15, z, biome);
    }

}


/// Used to calculate the index in a data array of `DATA_SIZE`.
#[inline]
fn calc_block_index(x: usize, y: usize, z: usize) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16, "x: {}, y: {}, z: {}", x, y, z);
    (x & 15) | ((z & 15) << 4) | ((y & 15) << 8)
}


#[inline]
fn calc_biome_2d_index(x: usize, z: usize) -> usize {
    debug_assert!(x < 16 && z < 16, "x: {}, z: {}", x, z);
    (x & 15) | ((z & 15) << 4)
}


#[inline]
fn calc_biome_3d_index(x: usize, y: usize, z: usize) -> usize {
    debug_assert!(x < 4 && y < 4 && z < 4, "x: {}, y: {}, z: {}", x, y, z);
    (x & 3) | ((z & 3) << 2) | ((y & 3) << 4)
}


/// A sub chunk, 16x16x16 blocks.
///
/// Actual sub chunk size: 4096*2 + 64 + 256 = 8512 bytes
pub struct SubChunk {
    world_info: Rc<WorldInfo>,
    blocks: Vec<u16>,
    /// In this biomes array, two type of biomes format coexists in order to reduce
    /// memory fragmentation of biomes, the first 64 bytes are used to map 3D biomes
    /// in a 4x4x4 cube where each byte stands for a cube of 4x4 blocks in the chunk.
    /// The next 256 bytes are for the legacy (pre 1.15) biomes 16x16 rectangle.
    biomes: Vec<u8>
}

impl SubChunk {

    pub fn new(world_info: Rc<WorldInfo>) -> Self {
        SubChunk {
            world_info,
            blocks: vec![0; DATA_SIZE],
            biomes: vec![0; 64 + 256]
        }
    }

    /// Return the world info this sub chunks belongs to.
    pub fn get_world_info(&self) -> &WorldInfo {
        &self.world_info
    }

    // RAW BLOCKS //

    /// Get the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    pub fn get_block_id(&self, x: usize, y: usize, z: usize) -> u16 {
        self.blocks[calc_block_index(x, y, z)]
    }

    /// Set the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The block id must valid for the registry of the world
    /// this chunk belongs to.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    pub fn set_block_id(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        self.blocks[calc_block_index(x, y, z)] = block_id;
    }

    // ACTUAL BLOCKS //

    /// Get the block at specific position by converting the resulting raw id to an
    /// actual block by using the world's block registry.
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        self.world_info.borrow().block_registry.0.get_from_id(self.get_block_id(x, y, z))
    }

    /// Set the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The block can be `None` to remove the block, if `Some`,
    /// the block must be valid for the registry of the world this chunk belongs to.
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Option<&Block>) {
        self.set_block_id(x, y, z, match block {
            Some(block) => block.get_id(),
            None => 0
        });
    }

    // RAW BIOMES //

    pub fn get_biome_2d_id(&self, x: usize, z: usize) -> u8 {
        self.biomes[64 + calc_biome_2d_index(x, z)]
    }

    pub fn get_biome_3d_id(&self, x: usize, y: usize, z: usize) -> u8 {
        // Note: Shifting position because biomes 3D cube is 4x4x4 and
        // I don't want to expose this details to the API.
        self.biomes[calc_biome_3d_index(x >> 2, y >> 2, z >> 2)]
    }

    pub fn set_biome_2d_id(&mut self, x: usize, z: usize, id: u8) {
        self.biomes[64 + calc_biome_2d_index(x, z)] = id;
    }

    pub fn set_biome_3d_id(&mut self, x: usize, y: usize, z: usize, id: u8) {
        self.biomes[calc_biome_3d_index(x >> 2, y >> 2, z >> 2)] = id;
    }

    // ACTUAL BIOMES //

    pub fn get_biome_2d(&self, x: usize, z: usize) -> Option<&Biome> {
        self.world_info.biome_registry.0.get_from_id(self.get_biome_2d_id(x, z))
    }

    pub fn get_biome_3d(&self, x: usize, y: usize, z: usize) -> Option<&Biome> {
        self.world_info.biome_registry.0.get_from_id(self.get_biome_3d_id(x, y, z))
    }

    pub fn set_biome_2d(&mut self, x: usize, z: usize, biome: &Biome) {
        self.set_biome_2d_id(x, z, biome.get_id());
    }

    pub fn set_biome_3d(&mut self, x: usize, y: usize, z: usize, biome: &Biome) {
        self.set_biome_3d_id(x, y, z, biome.get_id());
    }

    // LEGACY BIOMES //

    /// Get the biome id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// *The resolution of the biome grid allowed by this structure
    /// might not be the resolution of the internal Minecraft storage witch only allow
    /// 1024 biomes for a whole chunk (not sub chunk).*
    ///
    /// *In version prior to 1.15, the biome grid was a 2D rectangle, so the same biome
    /// is set for all Y.**
    #[deprecated]
    pub fn get_biome_id(&self, x: usize, y: usize, z: usize) -> u8 {
        self.biomes[calc_block_index(x, y, z)]
    }

    /// Get the biome at specific position by converting the resulting raw id to an
    /// actual biome by using a biome registry, this registry should be the one of
    /// the world this chunk belongs to.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    #[deprecated]
    pub fn get_biome(&self, x: usize, y: usize, z: usize) -> Option<&Biome> {
        self.world_info.biome_registry.0.get_from_id(self.get_biome_id(x, y, z))
    }

    /// Set the biome id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// Check SubChunk::get_biome_id function for more information.
    #[deprecated]
    pub fn set_biome_id(&mut self, x: usize, y: usize, z: usize, biome_id: u8) {
        self.biomes[calc_block_index(x, y, z)] = biome_id
    }

    /// Set the biome at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The biome can be `None` to remove the biome, if `Some`,
    /// the biome must be valid for the registry of the world this chunk belongs to.
    #[deprecated]
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: Option<&Biome>) {
        self.set_biome_id(x, y, z, match biome {
            Some(biome) => biome.get_id(),
            None => 0
        })
    }

}
