use crate::rand::jrand::JavaRandom;
use crate::world::ChunkMap;
use super::{Feature, PosDistrib};


pub struct RepeatedFeature {
    count: u16,
    feature: Box<dyn Feature>,
    pos_picker: Box<dyn PosDistrib>
}

impl Feature for RepeatedFeature {
    fn generate(&self, world: &mut ChunkMap, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        for _ in 0..self.count {
            let (rx, ry, rz) = self.pos_picker.pick_pos(world, rand, x, y, z);
            self.feature.generate(world, rand, rx, ry, rz);
        }
    }
}