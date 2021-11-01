use std::sync::Arc;
use mc_core::world::chunk::ChunkResult;
use mc_core::rand::JavaRandom;

pub mod distrib;
pub mod branch;
pub mod vein;
pub mod lake;
pub mod debug;
pub mod dungeon;
pub mod tree;

use distrib::{Distrib, DistribFeature, UniformVerticalDistrib, TriangularVerticalDistrib};
use branch::{RepeatedFeature, OptionalFeature, RepeatCount};

use mc_core::heightmap::HeightmapType;
use mc_core::block::BlockState;
use mc_core::biome::Biome;
use mc_core::world::level::LevelEnv;


/// Base trait for level features generators.
pub trait Feature {

    /// Generate the feature in this chunk, the x/y/z coordinates is where to generate it.
    ///
    /// When called from the biome decorator, `y=0` and x/z are the coordinates of the population
    /// chunk, a.k.a. the chunk with an offset of 8/8 blocks.
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool;

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
        self.distributed(UniformVerticalDistrib::new(min_y, max_y))
    }

    fn distributed_triangular(self, y_center: i32, y_spread: i32) -> DistribFeature<Self, TriangularVerticalDistrib>
    where
        Self: Sized
    {
        self.distributed(TriangularVerticalDistrib::new(y_center, y_spread))
    }

    fn repeated<C>(self, count: C) -> RepeatedFeature<Self, C>
    where
        Self: Sized,
        C: RepeatCount
    {
        RepeatedFeature::new(self, count)
    }

    /// Make the current feature actually generate 1 in a `bound` time.
    fn optional(self, bound: u16) -> OptionalFeature<Self, ()>
    where
        Self: Sized
    {
        OptionalFeature::new(self, (), bound)
    }

    fn optional_or<E>(self, bound: u16, else_feature: E) -> OptionalFeature<Self, E>
    where
        Self: Sized,
        E: Feature
    {
        OptionalFeature::new(self, else_feature, bound)
    }

}


impl Feature for () {
    fn generate(&self, _level: &mut dyn LevelView, _rand: &mut JavaRandom, _x: i32, _y: i32, _z: i32) -> bool {
        false
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

    pub fn generate(&self, chunk: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        for feature in &self.features {
            feature.generate(chunk, rand, x, y, z);
        }
    }

}


/// A local level view used to generate feature in an partial level view.
///
/// **Note that you can't directly access "chunks" in a level view. However,
/// you can check if a specific one exists or check its sub chunks.**
pub trait LevelView {

    fn get_env(&self) -> &Arc<LevelEnv>;

    fn has_sub_chunk(&self, cx: i32, cz: i32) -> bool;
    fn has_sub_chunk_at(&self, x: i32, z: i32) -> bool {
        self.has_sub_chunk(x >> 4, z >> 4)
    }

    fn set_block_at(&mut self, x: i32, y: i32, z: i32, state: &'static BlockState) -> ChunkResult<()>;
    fn get_block_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static BlockState>;

    fn get_biome_at(&self, x: i32, y: i32, z: i32) -> ChunkResult<&'static Biome>;

    fn get_heightmap_column_at(&self, heightmap_type: &'static HeightmapType, x: i32, z: i32) -> ChunkResult<i32>;

}
