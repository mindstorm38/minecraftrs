use mc_core::block::{Block, GlobalBlocks};
use mc_core::heightmap::HeightmapType;
use mc_core::rand::JavaRandom;

use mc_vanilla::block::material::TAG_LEAVES;
use mc_vanilla::block::AIR;

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


/// A uniform distribution between two vertical points, X and Z and picked randomly.
pub struct UniformVerticalDistrib<const LATE_Y: bool> {
    min_y: i32,
    max_y: i32
}

impl UniformVerticalDistrib<false> {
    pub fn new(min_y: i32, max_y: i32) -> Self {
        Self { min_y, max_y }
    }
}

impl UniformVerticalDistrib<true> {
    pub fn new_with_late_y(min_y: i32, max_y: i32) -> Self {
        Self { min_y, max_y }
    }
}

impl<const LATE_Y: bool> Distrib for UniformVerticalDistrib<LATE_Y> {
    fn pick_pos(&self, _level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let mut ry = 0;
        if !LATE_Y {
            ry = rand.next_int_bounded(self.max_y - self.min_y) + self.min_y;
        }
        let rz = z + rand.next_int_bounded(16);
        if LATE_Y {
            ry = rand.next_int_bounded(self.max_y - self.min_y) + self.min_y;
        }
        Some((rx, ry, rz))
    }
}


/// A triangular distribution with a center and a spread, X and Z are picked randomly.
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


/// A distribution that picks random X and Z, and then assign the height (of the given type)
/// to the Y coordinate (panicking if the heightmap type is not supported).
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


/// A distribution specific to lava lake.
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


/// A distribution that modifies the Y coordinate by lowering it until the given predicate
/// returns false for the block a `Y + offset`.
pub struct OffsetWhileDistrib<P> {
    min: i32,
    offset: i32,
    predicate: P
}

impl<P> OffsetWhileDistrib<P>
where
    P: Fn(&'static Block, &GlobalBlocks) -> bool
{
    pub fn new(min: i32, offset: i32, predicate: P) -> Self {
        Self { min, offset, predicate }
    }
}

impl OffsetWhileDistrib<fn(&'static Block, &GlobalBlocks) -> bool> {

    pub fn new_air_or_leaves() -> Self {
        fn is_air_or_leaves(block: &'static Block, blocks: &GlobalBlocks) -> bool {
            block == &AIR || blocks.has_block_tag(block, &TAG_LEAVES)
        }
        Self::new(0, 0, is_air_or_leaves)
    }

    pub fn new_air_below() -> Self {
        fn is_air(block: &'static Block, _blocks: &GlobalBlocks) -> bool {
            block == &AIR
        }
        Self::new(0, -1, is_air)
    }

}

impl<P> Distrib for OffsetWhileDistrib<P>
where
    P: Fn(&'static Block, &GlobalBlocks) -> bool
{
    fn pick_pos(&self, level: &mut dyn LevelView, _rand: &mut JavaRandom, x: i32, mut y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let env_blocks = &level.get_env().blocks;
        while y > self.min {
            let valid = match level.get_block_at(x, y + self.offset, z) {
                Ok(state) => (self.predicate)(state.get_block(), env_blocks),
                Err(_) => (self.predicate)(&AIR, env_blocks)  // Air by default
            };
            if valid {
                y -= 1;
            } else {
                break;
            }
        }
        Some((x, y, z))
    }
}
