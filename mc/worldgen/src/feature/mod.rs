use mc_core::rand::JavaRandom;
use mc_core::world::chunk::Chunk;

pub mod distrib;
pub mod repeated;
pub mod vein;


/// A safe wrapper around a chunk, this can be used to set block at real level coordinates
/// and ignore the action if it's outside of the chunk.
pub struct ChunkAccess<'a> {
    pub chunk: &'a mut Chunk,
    pub min_x: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_z: i32
}


pub trait Feature {

    /// Generate the feature in this chunk, the x/y/z coordinates is where to generate it.
    ///
    /// When called from the biome decorator, `y=0` and x/z are the coordinates of the population
    /// chunk, a.k.a. the chunk with an offset of 8/8 blocks.
    fn generate(&self, chunk: &mut ChunkAccess, rand: &mut JavaRandom, x: i32, y: i32, z: i32);

}
