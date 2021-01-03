use crate::block::{BlockRegistry, Block};
use crate::biome::{BiomeRegistry, Biome};
use crate::res::Registrable;


/// The number of blocks for each direction in sub chunks.
pub const SIZE: usize = 16;
/// The total count of data for a 3 dimensional cube of `SIZE`.
pub const DATA_SIZE: usize = SIZE * SIZE * SIZE;


/// Used to calculate the index in a data array of `DATA_SIZE`.
#[inline]
fn calc_data_index(x: u8, y: u8, z: u8) -> usize {
    debug_assert!(x < 16 && y < 16 && z < 16);
    (x as usize & 15) | ((y as usize & 15) << 4) | ((z as usize & 15) << 8)
}


/// A vertical chunk, 16x16
pub struct Chunk {
    cx: i32,
    cz: i32,
    sub_chunks: Vec<SubChunk>
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

    pub fn get_block<'a>(&self, x: u8, y: u8, z: u8, reg: &'a BlockRegistry) -> Option<&'a Block> {
        reg.get_from_id(self.blocks[calc_data_index(x, y, z)])
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block: &Block) {
        self.blocks[calc_data_index(x, y, z)] = block.get_id()
    }

    pub fn get_biome<'a>(&self, x: u8, y: u8, z: u8, reg: &'a BiomeRegistry) -> Option<&'a Biome> {
        reg.0.get_from_id(self.biomes[calc_data_index(x, y, z)])
    }

    pub fn set_biome<'a>(&mut self, x: u8, y: u8, z: u8, biome: &Biome) {
        self.biomes[calc_data_index(x, y, z)] = biome.get_id()
    }

}
