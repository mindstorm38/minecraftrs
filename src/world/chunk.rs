use crate::res::Registrable;
use crate::block::Block;
use crate::biome::Biome;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const DATA_SIZE: usize = SIZE * SIZE * SIZE;


/// Used to calculate the index in a data array of `DATA_SIZE`.
#[inline]
fn calc_data_index(x: usize, y: usize, z: usize) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16, "x: {}, y: {}, z: {}", x, y, z);
    (x & 15) | ((y & 15) << 4) | ((z & 15) << 8)
}


/// A vertical chunk, 16x16 blocks.
pub struct Chunk {
    cx: i32,
    cz: i32,
    sub_chunks: Vec<SubChunk>
}

impl Chunk {

    pub fn new(cx: i32, cz: i32, sub_chunks_count: u8) -> Self {
        Chunk {
            cx,
            cz,
            sub_chunks: (0..sub_chunks_count).map(|_| SubChunk::new()).collect()
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

    /// Set the block at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Option<&Block>) {
        self.get_sub_chunk_mut(y >> 4).set_block(x, y & 15, z, block);
    }

    /// Get the biome id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn get_biome_id(&self, x: usize, y: usize, z: usize) -> u8 {
        self.get_sub_chunk(y >> 4).get_biome_id(x, y & 15, z)
    }

    /// Set the biome id at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn set_biome_id(&mut self, x: usize, y: usize, z: usize, biome_id: u8) {
        self.get_sub_chunk_mut(y >> 4).set_biome_id(x, y & 15, z, biome_id);
    }

    /// Set the biome at a specific position relative to this chunk.
    /// **The function may panic if positions are not within ranges,
    /// for safer function, check world functions.**
    /// *For more documentation, refer to SubChunk structure's function
    /// with the same name.*
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: Option<&Biome>) {
        self.get_sub_chunk_mut(y >> 4).set_biome(x, y & 15, z, biome);
    }

}


/// A sub chunk, 16x16x16 blocks.
pub struct SubChunk {
    pub blocks: Vec<u16>,
    pub biomes: Vec<u8>
}

impl SubChunk {

    pub fn new() -> Self {
        SubChunk {
            blocks: vec![0; DATA_SIZE],
            biomes: vec![0; DATA_SIZE]
        }
    }

    /// Get the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    pub fn get_block_id(&self, x: usize, y: usize, z: usize) -> u16 {
        self.blocks[calc_data_index(x, y, z)]
    }

    /// Set the block id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The block id must valid for the registry of the world
    /// this chunk belongs to.
    ///
    /// **This function is intended for internal uses, but is still public to allow
    /// low level manipulation, be careful!**
    pub fn set_block_id(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        self.blocks[calc_data_index(x, y, z)] = block_id;
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

    /// Get the biome id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// *The resolution of the biome grid allowed by this structure
    /// might not be the resolution of the internal Minecraft storage witch only allow
    /// 1024 chunks for a whole chunk (not sub chunk).*
    ///
    /// *In version prior to 1.15, the biome grid was a 2D rectangle, so the same biome
    /// is set for all Y.**
    pub fn get_biome_id(&self, x: usize, y: usize, z: usize) -> u8 {
        self.biomes[calc_data_index(x, y, z)]
    }

    /// Set the biome id at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`.
    ///
    /// Check SubChunk::get_biome_id function for more information.
    pub fn set_biome_id(&mut self, x: usize, y: usize, z: usize, biome_id: u8) {
        self.biomes[calc_data_index(x, y, z)] = biome_id
    }

    /// Set the biome at specific position, the position is relative to this chunk
    /// `(0 <= x/y/z < 16)`. The biome can be `None` to remove the biome, if `Some`,
    /// the biome must be valid for the registry of the world this chunk belongs to.
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: Option<&Biome>) {
        self.set_biome_id(x, y, z, match biome {
            Some(biome) => biome.get_id(),
            None => 0
        })
    }

}
