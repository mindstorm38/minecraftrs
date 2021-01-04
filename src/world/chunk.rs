use crate::block::{BlockRegistry, Block};
use crate::biome::{BiomeRegistry, Biome};
use crate::res::Registrable;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const DATA_SIZE: usize = SIZE * SIZE * SIZE;


/// Used to calculate the index in a data array of `DATA_SIZE`.
#[inline]
fn calc_data_index(x: usize, y: usize, z: usize) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16);
    (x & 15) | ((y & 15) << 4) | ((z & 15) << 8)
}


/// A vertical chunk, 16x16
pub struct Chunk {
    cx: i32,
    cz: i32,
    sub_chunks: Vec<SubChunk>
}

impl Chunk {

    pub fn new(sub_chunks_count: u8) -> Self {
        Chunk {
            cx: 0,
            cz: 0,
            sub_chunks: (0..sub_chunks_count).map(|_| SubChunk::new()).collect()
        }
    }

    pub fn get_sub_chunk(&self, cy: usize) -> &SubChunk {
        &self.sub_chunks[cy]
    }

    pub fn get_sub_chunk_mut(&mut self, cy: usize) -> &mut SubChunk {
        &mut self.sub_chunks[cy]
    }

}


/// A sub chunk, 16x16x16
pub struct SubChunk {
    blocks: Vec<u16>,
    biomes: Vec<u8>
}

impl SubChunk {

    pub fn new() -> Self {
        SubChunk {
            blocks: vec![0; DATA_SIZE],
            biomes: vec![0; DATA_SIZE]
        }
    }

    pub fn get_block<'a>(&self, x: usize, y: usize, z: usize, reg: &'a BlockRegistry) -> Option<&'a Block> {
        reg.get_from_id(self.blocks[calc_data_index(x, y, z)])
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Option<&Block>) {
        self.blocks[calc_data_index(x, y, z)] = match block {
            Some(block) => block.get_id(),
            None => 0
        }
    }

    pub fn get_biome<'a>(&self, x: usize, y: usize, z: usize, reg: &'a BiomeRegistry) -> Option<&'a Biome> {
        reg.0.get_from_id(self.biomes[calc_data_index(x, y, z)])
    }

    pub fn set_biome<'a>(&mut self, x: usize, y: usize, z: usize, biome: &Biome) {
        self.biomes[calc_data_index(x, y, z)] = biome.get_id()
    }

}
