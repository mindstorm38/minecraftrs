use mc_core::rand::JavaRandom;

use crate::view::LevelView;

pub mod distrib;
pub mod branch;
pub mod vein;
pub mod lake;
pub mod debug;
pub mod dungeon;
pub mod tree;
pub mod flower;

use distrib::{Distrib, DistribFeature, TriangularVerticalDistrib, UniformVerticalDistrib};
use branch::{OptionalFeature, RepeatCount, RepeatedFeature, ChainFeature};

/// Base trait for level features generators.
pub trait Feature {

    /// Generate the feature in this chunk, the x/y/z coordinates is where to generate it.
    ///
    /// When called from the biome decorator, `y=0` and x/z are the coordinates of the population
    /// chunk, a.k.a. the chunk with an offset of 8/8 blocks.
    ///
    /// The caller must ensure that the given level view provides at least 2 by 2 chunks, with the
    /// feature chunk being centered on the center of these 4 chunks.
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool;

    fn distributed<D: Distrib>(self, distrib: D) -> DistribFeature<Self, D>
    where
        Self: Sized
    {
        DistribFeature::new(self, distrib)
    }

    fn distributed_uniform(self, min_y: i32, max_y: i32) -> DistribFeature<Self, UniformVerticalDistrib<false>>
    where
        Self: Sized
    {
        self.distributed(UniformVerticalDistrib::new(min_y, max_y))
    }

    fn distributed_uniform_with_late_y(self, min_y: i32, max_y: i32) -> DistribFeature<Self, UniformVerticalDistrib<true>>
    where
        Self: Sized
    {
        self.distributed(UniformVerticalDistrib::new_with_late_y(min_y, max_y))
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

    fn chain<B>(self, b_feature: B) -> ChainFeature<Self, B>
    where
        Self: Sized,
        B: Feature
    {
        ChainFeature::new(self, b_feature)
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
