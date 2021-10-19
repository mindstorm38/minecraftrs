use mc_core::world::chunk::ChunkGuard;
use mc_core::rand::JavaRandom;
use super::Feature;


/// A trait to implement on feature distribution structures to use later in composed structures.
pub trait Distrib {
    fn pick_pos(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> Option<(i32, i32, i32)>;
}


/// A feature that implements a distribution function to call a specific feature.
pub struct DistribFeature<F: Feature, D: Distrib> {
    feature: F,
    distrib: D,
}

impl<F: Feature, D: Distrib> DistribFeature<F, D> {
    pub fn new(feature: F, distrib: D) -> Self {
        Self {
            feature,
            distrib
        }
    }
}

impl<F: Feature> DistribFeature<F, UniformVerticalDistrib> {
    pub fn new_uniform_vertical(feature: F, min_y: i32, max_y: i32) -> Self {
        Self::new(feature, UniformVerticalDistrib::new(min_y, max_y))
    }
}

impl<F: Feature> DistribFeature<F, TriangularVerticalDistrib> {
    pub fn new_triangular_vertical(feature: F, y_center: i32, y_spread: i32) -> Self {
        Self::new(feature, TriangularVerticalDistrib::new(y_center, y_spread))
    }
}

impl<F: Feature, D: Distrib> Feature for DistribFeature<F, D> {
    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {
        if let Some((rx, ry, rz)) = self.distrib.pick_pos(chunk, rand, x, y, z) {
            self.feature.generate(chunk, rand, rx, ry, rz);
        }
    }
}


pub struct UniformVerticalDistrib {
    min_y: i32,
    max_y: i32
}

impl UniformVerticalDistrib {
    pub fn new(min_y: i32, max_y: i32) -> Self {
        UniformVerticalDistrib { min_y, max_y }
    }
}

impl Distrib for UniformVerticalDistrib {
    fn pick_pos(&self, _chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let ry = rand.next_int_bounded(self.max_y - self.min_y) + self.min_y;
        let rz = z + rand.next_int_bounded(16);
        Some((rx, ry, rz))
    }
}


pub struct TriangularVerticalDistrib {
    y_center: i32,
    y_spread: i32
}

impl TriangularVerticalDistrib {
    pub fn new(y_center: i32, y_spread: i32) -> Self {
        TriangularVerticalDistrib { y_center, y_spread }
    }
}

impl Distrib for TriangularVerticalDistrib {
    fn pick_pos(&self, _chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let ry = rand.next_int_bounded(self.y_spread) + rand.next_int_bounded(self.y_spread) + self.y_center - self.y_spread;
        let rz = z + rand.next_int_bounded(16);
        Some((rx, ry, rz))
    }
}
