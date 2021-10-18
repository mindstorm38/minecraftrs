use mc_core::rand::JavaRandom;
use mc_core::world::chunk::{Chunk, ChunkGuard};

pub mod distrib;
pub mod repeated;
pub mod vein;


pub trait Feature {

    /// Generate the feature in this chunk, the x/y/z coordinates is where to generate it.
    ///
    /// When called from the biome decorator, `y=0` and x/z are the coordinates of the population
    /// chunk, a.k.a. the chunk with an offset of 8/8 blocks.
    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32);

}
