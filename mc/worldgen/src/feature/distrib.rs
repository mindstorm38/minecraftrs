use mc_core::heightmap::HeightmapType;
use mc_core::rand::JavaRandom;

use crate::view::LevelView;
use super::Feature;


/// A trait to implement on feature distribution structures to use later in composed structures.
pub trait Distrib {
    fn pick_pos(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> Option<(i32, i32, i32)>;
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

impl<F: Feature, D: Distrib> Feature for DistribFeature<F, D> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        if let Some((rx, ry, rz)) = self.distrib.pick_pos(level, rand, x, y, z) {
            self.feature.generate(level, rand, rx, ry, rz)
        } else {
            false
        }
    }
}


pub struct UniformVerticalDistrib {
    min_y: i32,
    max_y: i32
}

impl UniformVerticalDistrib {
    pub fn new(min_y: i32, max_y: i32) -> Self {
        Self { min_y, max_y }
    }
}

impl Distrib for UniformVerticalDistrib {
    fn pick_pos(&self, _level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
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
        Self { y_center, y_spread }
    }
}

impl Distrib for TriangularVerticalDistrib {
    fn pick_pos(&self, _level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let ry = rand.next_int_bounded(self.y_spread) + rand.next_int_bounded(self.y_spread) + self.y_center - self.y_spread;
        let rz = z + rand.next_int_bounded(16);
        Some((rx, ry, rz))
    }
}


pub struct HeightmapDistrib {
    heightmap_type: &'static HeightmapType
}

impl HeightmapDistrib {
    pub fn new(heightmap_type: &'static HeightmapType) -> Self {
        Self { heightmap_type }
    }
}

impl Distrib for HeightmapDistrib {
    fn pick_pos(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let rz = z + rand.next_int_bounded(16);
        Some((rx, level.get_heightmap_column_at(self.heightmap_type, rx, rz).unwrap(), rz))
    }
}


pub struct LavaLakeDistrib;

impl Distrib for LavaLakeDistrib {
    fn pick_pos(&self, _level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let ry = {
            let ry = rand.next_int_bounded(120) + 8;
            rand.next_int_bounded(ry)
        };
        let rz = z + rand.next_int_bounded(16);
        if ry < 63 || rand.next_int_bounded(10) == 0 {
            Some((rx, ry, rz))
        } else {
            None
        }
    }
}
