//! Tree-related features for Minecraft, these algorithms might seems a little weird
//! because they are translated from a Java decompilation by MCP. The main goal is to
//! be as accurate as possible.

use mc_core::block::{Block, BlockState, GlobalBlocks};
use mc_core::rand::JavaRandom;
use mc_core::pos::Axis;

use mc_vanilla::block::material::TAG_NON_SOLID;
use mc_vanilla::block::*;

use super::LevelView;

mod common;
mod taiga;
mod jungle;
mod big;

pub use common::*;
pub use taiga::*;
pub use jungle::*;
pub use big::*;


/// Height generator trait for trees, default implementation are for `i32` (const), single random
/// and then double random.
pub trait TreeHeight {
    fn gen(&self, rand: &mut JavaRandom) -> i32;
}

// Constant-height.
impl TreeHeight for i32 {
    fn gen(&self, _rand: &mut JavaRandom) -> i32 {
        *self
    }
}

/// A basic random tree height with an offset and a bound (exclusive) to the random value that
/// is added to the offset.
pub struct RandomTreeHeight {
    offset: i32,
    bound: i32
}

impl RandomTreeHeight {
    pub fn new(offset: i32, bound: i32) -> Self {
        Self { offset, bound }
    }
}

impl TreeHeight for RandomTreeHeight {
    fn gen(&self, rand: &mut JavaRandom) -> i32 {
        self.offset + rand.next_int_bounded(self.bound)
    }
}

/// A double random tree height with an offset and two bounds (both exclusive) for the two random
/// values that are added to the offset.
pub struct DoubleRandomTreeHeight {
    offset: i32,
    bound0: i32,
    bound1: i32
}

impl DoubleRandomTreeHeight {
    pub fn new(offset: i32, bound0: i32, bound1: i32) -> Self {
        Self { offset, bound0, bound1 }
    }
}

impl TreeHeight for DoubleRandomTreeHeight {
    fn gen(&self, rand: &mut JavaRandom) -> i32 {
        self.offset + rand.next_int_bounded(self.bound0) + rand.next_int_bounded(self.bound1)
    }
}


/// A common useful structure storing log and leaves block states used for a tree,
/// this struct provides a method to get leaves state with specific distance.
pub struct TreePalette {
    log: &'static BlockState,
    leaves: &'static BlockState
}

impl TreePalette {

    pub fn new(log: &'static Block, leaves: &'static Block) -> Self {
        Self {
            log: log.get_default_state().with(&PROP_AXIS, Axis::Y).unwrap(),
            leaves: leaves.get_default_state().with(&PROP_PERSISTENT, false).unwrap()
        }
    }

    pub fn new_oak() -> Self {
        Self::new(&OAK_LOG, &OAK_LEAVES)
    }

    pub fn new_birch() -> Self {
        Self::new(&BIRCH_LOG, &BIRCH_LEAVES)
    }

    pub fn new_spruce() -> Self {
        Self::new(&SPRUCE_LOG, &SPRUCE_LEAVES)
    }

    pub fn new_jungle() -> Self {
        Self::new(&JUNGLE_LOG, &JUNGLE_LEAVES)
    }

    pub fn get_leaves(&self, distance: u8) -> &'static BlockState {
        self.leaves.with(&PROP_LEAVES_DISTANCE, distance).unwrap()
    }

}


/// Internal method to add a falling vines column.
pub fn generate_falling_vines(level: &mut dyn LevelView, x: i32, mut y: i32, z: i32, state: &'static BlockState) {
    for _ in 0..5 {
        level.set_block_at(x, y, z, state).unwrap();
        y -= 1;
        if !level.get_block_at(x, y, z).unwrap().is_block(&AIR) {
            break;
        }
    }
}


/// Internal common function to generate a leaves horizontal layer. The layer is a square with
/// blocks in the corner depending on the "corner predicate", this function replaces non-solid
/// blocks.
pub fn generate_leaves_layer<F: FnMut() -> bool>(
    level: &mut dyn LevelView,
    env_blocks: &GlobalBlocks,
    x: i32, y: i32, z: i32,
    radius: i32,
    state: &'static BlockState,
    mut corner_predicate: F
) {
    for bx in (x - radius)..=(x + radius) {
        let dx = (bx - x).abs();
        for bz in (z - radius)..=(z + radius) {
            let dz = (bz - z).abs();
            if dx != radius || dz != radius || corner_predicate() {
                // FIXME: The real condition is last block not opaque.
                let current_block = level.get_block_at(bx, y, bz).unwrap().get_block();
                if env_blocks.has_block_tag(current_block, &TAG_NON_SOLID) {
                    level.set_block_at(bx, y, bz, state).unwrap();
                }
            }
        }
    }
}


/// An array from which you can get secondary and tertiary axis from a given one.
/// For example if we have a coordinate triplet `[x, y, z]` (X has index 0, Y has
/// index 1 and Z has index 2).
///
/// So if you want to get the secondary axis of X (index 0), `COORD_PAIRS[0] = 2`,
/// which is the index of Z, and to get the tertiary axis, offset by 3 like this:
/// `COORD_PAIRS[0 + 3] = 1` which is the index of Y.
pub static COORD_PAIRS: [usize; 6] = [2, 0, 0, 1, 2, 1];


/// A special iterator usable to make clean line of block from a point A to point B.
/// This structure might be moved to a common utility module in the future.
pub struct BlockLineIter {
    from: [i32; 3],
    primary_idx: usize,
    secondary_idx: usize,
    tertiary_idx: usize,
    primary_increment: i32,
    secondary_factor: f64,
    tertiary_factor: f64,
    offset: f64,
    step: i32,
    step_limit: i32
}

impl BlockLineIter {
    fn new(from: [i32; 3], to: [i32; 3], offset: f64) -> Self {

        let mut diffs = [0; 3];
        let mut primary_diff = 0i32;
        let mut primary_idx = 0;

        for i in 0..3 {
            let diff = to[i] - from[i];
            diffs[i] = diff;
            if diff.abs() > primary_diff.abs() {
                primary_diff = diff;
                primary_idx = i;
            }
        }

        if primary_diff == 0 {
            return Self {
                from,
                primary_idx: 0,
                secondary_idx: 0,
                tertiary_idx: 0,
                primary_increment: 0,
                secondary_factor: 0.0,
                tertiary_factor: 0.0,
                offset: 0.0,
                step: 0,
                step_limit: 0,
            };
        }

        let secondary_idx = COORD_PAIRS[primary_idx];
        let tertiary_idx = COORD_PAIRS[primary_idx + 3];
        let primary_increment = if primary_diff > 0 { 1 } else { -1 };

        let secondary_factor = diffs[secondary_idx] as f64 / primary_diff as f64;
        let tertiary_factor = diffs[tertiary_idx] as f64 / primary_diff as f64;

        let step_limit = primary_diff + primary_increment;

        Self {
            from,
            primary_idx,
            secondary_idx,
            tertiary_idx,
            primary_increment,
            secondary_factor,
            tertiary_factor,
            offset,
            step: 0,
            step_limit,
        }

    }
}

impl Iterator for BlockLineIter {
    type Item = (i32, i32, i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        let step = self.step;
        if step != self.step_limit {
            let mut coords = [0; 3];
            coords[self.primary_idx] = self.from[self.primary_idx] + step;
            coords[self.secondary_idx] = (self.from[self.secondary_idx] as f64 + (step as f64 * self.secondary_factor) + self.offset).floor() as i32;
            coords[self.tertiary_idx] = (self.from[self.tertiary_idx] as f64 + (step as f64 * self.tertiary_factor) + self.offset).floor() as i32;
            self.step += self.primary_increment;
            Some((coords[0], coords[1], coords[2], self.step))
        } else {
            None
        }
    }
}
