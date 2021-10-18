use mc_core::world::chunk::ChunkGuard;
use mc_core::rand::JavaRandom;

use super::Feature;
use super::distrib::Distrib;


pub struct RepeatedFeature<F: Feature, D: Distrib> {
    count: u16,
    feature: F,
    distrib: D
}

impl<F: Feature, D: Distrib> Feature for RepeatedFeature<F, D> {

    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        for _ in 0..self.count {
            if let Some((rx, ry, rz)) = self.distrib.pick_pos(chunk, rand, x, y, z) {
                self.feature.generate(chunk, rand, rx, ry, rz);
            }
        }
    }

}
