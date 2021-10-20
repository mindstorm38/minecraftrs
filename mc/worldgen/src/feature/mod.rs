use mc_core::world::chunk::{Chunk, ChunkGuard, ChunkResult};
use mc_core::rand::JavaRandom;

pub mod distrib;
pub mod repeated;
pub mod vein;
pub mod lake;

use distrib::{Distrib, DistribFeature, UniformVerticalDistrib, TriangularVerticalDistrib};
use repeated::RepeatedFeature;
use mc_core::block::BlockState;


/// Base trait for level features generators.
pub trait Feature {

    /// Generate the feature in this chunk, the x/y/z coordinates is where to generate it.
    ///
    /// When called from the biome decorator, `y=0` and x/z are the coordinates of the population
    /// chunk, a.k.a. the chunk with an offset of 8/8 blocks.
    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32);

    fn distributed<D: Distrib>(self, distrib: D) -> DistribFeature<Self, D>
    where
        Self: Sized
    {
        DistribFeature::new(self, distrib)
    }

    fn distributed_uniform(self, min_y: i32, max_y: i32) -> DistribFeature<Self, UniformVerticalDistrib>
    where
        Self: Sized
    {
        DistribFeature::new_uniform_vertical(self, min_y, max_y)
    }

    fn distributed_triangular(self, y_center: i32, y_spread: i32) -> DistribFeature<Self, TriangularVerticalDistrib>
    where
        Self: Sized
    {
        DistribFeature::new_triangular_vertical(self, y_center, y_spread)
    }

    fn repeated(self, count: u16) -> RepeatedFeature<Self>
    where
        Self: Sized
    {
        RepeatedFeature::new(count, self)
    }

}


pub struct FeatureChain {
    features: Vec<Box<(dyn Feature + Send + Sync)>>
}

impl FeatureChain {

    pub fn new() -> Self {
        Self {
            features: Vec::new()
        }
    }

    pub fn push<F: Feature + Send + Sync + 'static>(&mut self, feature: F) {
        self.features.push(Box::new(feature));
    }

    pub fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        for feature in &self.features {
            feature.generate(chunk, rand, x, y, z);
        }
    }

}


/// A local level view used to generate feature in an partial level view.
pub trait LevelView {
    fn set_block_at(&mut self, x: i32, y: i32, z: i32, block: &'static BlockState) -> ChunkResult<()>;
    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState>;
}
