use crate::rand::jrand::JavaRandom;
use crate::world::ChunkMap;
use super::{Feature, PosDistrib};


pub struct RepeatedFeature {
    count: u16,
    feature: Box<dyn Feature>,
    distrib: Box<dyn PosDistrib>
}

impl Feature for RepeatedFeature {
    fn generate(&self, world: &mut ChunkMap, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        for _ in 0..self.count {
            if let Some((rx, ry, rz)) = self.distrib.pick_pos(world, rand, x, y, z) {
                self.feature.generate(world, rand, rx, ry, rz);
            }
        }
    }
}
