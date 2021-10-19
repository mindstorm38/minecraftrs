use mc_core::world::chunk::ChunkGuard;
use mc_core::rand::JavaRandom;

use super::Feature;


pub struct RepeatedFeature<F: Feature> {
    count: u16,
    feature: F
}

impl<F: Feature> RepeatedFeature<F> {
    pub fn new(count: u16, feature: F) -> Self {
        Self {
            count,
            feature
        }
    }
}

impl<F: Feature> Feature for RepeatedFeature<F> {
    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        for _ in 0..self.count {
            self.feature.generate(chunk, rand, x, y, z);
        }
    }
}
