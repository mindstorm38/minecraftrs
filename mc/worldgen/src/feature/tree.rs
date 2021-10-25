//! Tree-related features for Minecraft, these algorithms might seems a little weird
//! because they are translated a Java decompilation by MCP. The main goal is to be
//! as accurate as possible.

use std::sync::Arc;

use mc_core::block::{Block, BlockState};
use mc_core::world::level::LevelEnv;
use mc_core::rand::JavaRandom;
use mc_core::math::JAVA_PI;
use mc_core::pos::Axis;

use mc_vanilla::block::*;
use mc_vanilla::block::material::{TAG_LOG, TAG_LEAVES};

use crate::feature::LevelView;
use super::Feature;


pub struct TreeFeature {
    log_block: &'static BlockState,
    leaves_block: &'static BlockState,
    base_height: u16,
    with_vines: bool
}

impl TreeFeature {

    pub fn new(log_block: &'static Block, leaves_block: &'static Block, base_height: u16, with_vines: bool) -> Self {
        Self {
            log_block: log_block.get_default_state().with(&PROP_AXIS, Axis::Y).unwrap(),
            leaves_block: leaves_block.get_default_state()
                .with(&PROP_LEAVES_DISTANCE, 1).unwrap()
                .with(&PROP_PERSISTENT, true).unwrap(),
            base_height,
            with_vines
        }
    }

}

impl Feature for TreeFeature {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {

        let height = rand.next_int_bounded(3) + self.base_height as i32;

        if y < 1 || y + height + 1 > 256 {
            return false;
        }

        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;

        for dy in y..=(y + height + 1) {

            let radius = if dy == y {
                0
            } else if dy >= (y + height -1) {
                2
            } else {
                1
            };

            for dx in (x - radius)..=(x + radius) {
                for dz in (z - radius)..=(z + radius) {
                    if dy >= 0 && dy < 256 { // This condition seems useless since Y is already checked.
                        let block = level.get_block_at(dx, dy, dz).unwrap().get_block();
                        if block != &AIR && block != &DIRT && !env_blocks.has_block_tag(block, &TAG_LEAVES) && !env_blocks.has_block_tag(block, &TAG_LOG) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }

        }

        let ground_block = level.get_block_at(x, y - 1, z).unwrap().get_block();

        if ground_block != &GRASS_BLOCK && ground_block != &DIRT || y >= 256 - height - 1 { // Last condition seems redundant.
            return false;
        }

        level.set_block_at(x, y - 1, z, DIRT.get_default_state());

        for dy in (y + height - 3)..=(y + height) {

            let top_diff = dy - (y + height);
            let radius = 1 - top_diff / 2;

            for dx in (x - radius)..=(x + radius) {
                let x_diff = (dx - x).abs();
                for dz in (z - radius)..=(z + radius) {
                    let z_diff = (dz - z).abs();
                    if x_diff != radius || z_diff != radius || (rand.next_int_bounded(2) != 0 && top_diff != 0) /* TODO: && is last block not opaque */ {
                        level.set_block_at(dx, dy, dz, self.leaves_block).unwrap();
                    }
                }
            }

        }

        let block_vine = VINE.get_default_state();
        let block_west_vine = block_vine.with(&PROP_WEST, true).unwrap();
        let block_east_vine = block_vine.with(&PROP_EAST, true).unwrap();
        let block_south_vine = block_vine.with(&PROP_SOUTH, true).unwrap();
        let block_north_vine = block_vine.with(&PROP_NORTH, true).unwrap();

        for dy in y..(y + height) {
            let block = level.get_block_at(x, dy, z).unwrap().get_block();
            if block == &AIR || env_blocks.has_block_tag(block, &TAG_LEAVES) {

                level.set_block_at(x, dy, z, self.log_block).unwrap();

                if self.with_vines && dy != y {

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x - 1, dy, z).unwrap().is_block(&AIR) {
                        level.set_block_at(x - 1, dy, z, block_west_vine);
                    }

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x + 1, dy, z).unwrap().is_block(&AIR) {
                        level.set_block_at(x + 1, dy, z, block_east_vine);
                    }

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x, dy, z - 1).unwrap().is_block(&AIR) {
                        level.set_block_at(x, dy, z - 1, block_south_vine);
                    }

                    if rand.next_int_bounded(3) != 0 && level.get_block_at(x, dy, z + 1).unwrap().is_block(&AIR) {
                        level.set_block_at(x, dy, z + 1, block_north_vine);
                    }

                }

            }
        }

        if self.with_vines {

            for dy in (y + height - 3)..=(y + height) {

                let top_diff = dy - (y + height);
                let radius = 1 - top_diff / 2;

                for dx in (x - radius)..=(x + radius) {
                    for dz in (z - radius)..=(z + radius) {
                        if env_blocks.has_block_tag(level.get_block_at(dx, dy, dz).unwrap().get_block(), &TAG_LEAVES) {

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(x - 1, dy, z).unwrap().is_block(&AIR) {
                                fill_falling_vines(level, x - 1, dy, z, block_west_vine);
                            }

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(x + 1, dy, z).unwrap().is_block(&AIR) {
                                fill_falling_vines(level, x + 1, dy, z, block_east_vine);
                            }

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(x, dy, z - 1).unwrap().is_block(&AIR) {
                                fill_falling_vines(level, x, dy, z - 1, block_south_vine);
                            }

                            if rand.next_int_bounded(4) == 0 && level.get_block_at(x, dy, z + 1).unwrap().is_block(&AIR) {
                                fill_falling_vines(level, x, dy, z + 1, block_north_vine);
                            }

                        }
                    }
                }

            }

        }

        true

    }

}


pub struct BigTreeFeature {
    height_limit: BigTreeHeight,
    height_attenuation: f64,
    branch_density: f64,
    branch_slope: f64,
    scale_width: f64,
    leaf_density: f64,
    leaf_dist_limit: u16,
    log_block: &'static BlockState,
    leaves_block: &'static BlockState,
}

impl Default for BigTreeFeature {
    fn default() -> Self {
        Self {
            height_limit: BigTreeHeight::Random(5, 12),
            height_attenuation: 0.61799999999999999,
            branch_density: 1.0,
            branch_slope: 0.38100000000000001,
            scale_width: 1.0,
            leaf_density: 1.0,
            leaf_dist_limit: 5,
            log_block: OAK_LOG.get_default_state().with(&PROP_AXIS, Axis::Y).unwrap(),
            leaves_block: OAK_LEAVES.get_default_state().with(&PROP_LEAVES_DISTANCE, 1).unwrap()
        }
    }
}

pub enum BigTreeHeight {
    Const(u16),
    Random(u16, u16)
}

impl Feature for BigTreeFeature {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        BigTreeBuilder {
            rand: JavaRandom::new(rand.next_long()),
            feature: self,
            env: Arc::clone(level.get_env()),
            level,
        }.generate(x, y, z)
    }
}


static BIG_TREE_COORD_PAIRS: [usize; 6] = [2, 0, 0, 1, 2, 1];

struct BigTreeBuilder<'a, 'b> {
    rand: JavaRandom,
    feature: &'a BigTreeFeature,
    env: Arc<LevelEnv>,
    level: &'b mut dyn LevelView,
}

impl<'a, 'b> BigTreeBuilder<'a, 'b> {

    fn generate(&mut self, x: i32, y: i32, z: i32) -> bool {

        let base_height = match self.is_valid_position(x, y, z) {
            Ok(v) => v,
            _ => return false
        };

        let (height, leaf_nodes) = self.generate_leaf_nodes(x, y, z, base_height);

        self.generate_leaves(&leaf_nodes);
        self.generate_trunk(x, y, z, height);
        self.generate_leaves_branches(&leaf_nodes, base_height);

        true

    }

    /// Check if the base position of the trunk would allow a big tree to generate,
    /// returning `Err(())` if it is impossible or `Ok(base_height)`.
    fn is_valid_position(&mut self, x: i32, y: i32, z: i32) -> Result<u16, ()> {

        let mut base_height = match self.feature.height_limit {
            BigTreeHeight::Const(limit) => limit,
            BigTreeHeight::Random(offset, limit) => {
                offset + self.rand.next_int_bounded(limit as i32) as u16
            }
        };

        let ground_block = self.level.get_block_at(x, y - 1, z).unwrap().get_block();
        if ground_block != &GRASS_BLOCK && ground_block != &DIRT {
            return Err(());
        }

        let trunk_from = [x, y, z];
        let trunk_to = [x, y + base_height as i32, z];
        let trunk_length = self.count_block_line(trunk_from, trunk_to);

        if trunk_length == -1 {
            Ok(base_height)
        } else if trunk_length < 6 {
            Err(())
        } else {
            Ok(trunk_length as u16)
        }

    }

    fn generate_leaf_nodes(&mut self, x: i32, y: i32, z: i32, base_height: u16) -> (u16, Vec<(i32, i32, i32, i32)>) {

        let mut height = (base_height as f64 * self.feature.height_attenuation) as u16;
        if height >= base_height {
            height = base_height - 1;
        }

        let a = ((1.3819999999999999 + ((self.feature.leaf_density * base_height as f64) / 13.0).powf(2.0)) as i32).max(1) as usize;

        let mut leaf_nodes = Vec::with_capacity(a * base_height as usize);
        let mut leaf_offset = base_height as i32 - self.feature.leaf_dist_limit as i32;
        let mut dy = y + leaf_offset;
        let leaf_start = y + height as i32;

        leaf_nodes.push((x, dy, z, leaf_start));
        dy -= 1;

        while leaf_offset >= 0 {

            let layer_size = Self::calc_layer_size(leaf_offset, base_height);

            if layer_size >= 0.0 {

                for _ in 0..a {

                    let length = self.feature.scale_width * (layer_size as f64 * (self.rand.next_float() as f64 + 0.32800000000000001));
                    let angle = self.rand.next_float() as f64 * 2.0 * JAVA_PI;

                    let dx = (length * angle.sin() + x as f64 + 0.5).floor() as i32;
                    let dz = (length * angle.cos() + z as f64 + 0.5).floor() as i32;

                    let branch_from = [dx, dy, dz];
                    let branch_to = [dx, dy + self.feature.leaf_dist_limit as i32, dz];

                    if self.count_block_line(branch_from, branch_to) == -1 {

                        let horiz_dist = (((x - dx).pow(2) + (z - dz).pow(2)) as f64).sqrt() * self.feature.branch_slope;

                        let branch_dy = if dy as f64 - horiz_dist > leaf_start as f64 {
                            leaf_start
                        } else {
                            (dy as f64 - horiz_dist) as i32
                        };

                        let mut branch_from = [dx, branch_dy, dz];
                        let mut branch_to = [dx, dy, dz];

                        if self.count_block_line(branch_from, branch_to) == -1 {
                            leaf_nodes.push((dx, dy, dz, branch_dy));
                        }

                    }

                }

            }

            dy -= 1;
            leaf_offset -= 1;

        }

        (height, leaf_nodes)

    }

    /// Generate all leaf nodes.
    fn generate_leaves(&mut self, nodes: &Vec<(i32, i32, i32, i32)>) {
        for &(x, y, z, _) in nodes {
            self.generate_leaf_node(x, y, z);
        }
    }

    /// Generate one leaf node, it will generate each leaves layer.
    fn generate_leaf_node(&mut self, x: i32, y: i32, z: i32) {
        let y_limit = y + self.feature.leaf_dist_limit as i32;
        for by in y..y_limit {
            let radius = if by != y && by != y_limit - 1 { 3.0 } else { 2.0 };
            self.generate_leaves_layer(x, by, z, radius);
        }
    }

    /// Generate an horizontal circle of leaves at given position and radius.
    fn generate_leaves_layer(&mut self, x: i32, y: i32, z: i32, radius: f32) {

        let radius_f64 = radius as f64;
        let radius_int = (radius_f64 + 0.61799999999999999) as i32;

        for dx in -radius_int..=radius_int {
            for dz in -radius_int..=radius_int {
                let dist = ((dx as f64 + 0.5).powf(2.0) + (dz as f64 + 0.5).powf(2.0)).sqrt();
                if dist <= radius_f64 {
                    let bx = x + dx;
                    let bz = z + dz;
                    let block = self.level.get_block_at(bx, y, bz).unwrap().get_block();
                    if block == &AIR || self.env.blocks.has_block_tag(block, &TAG_LEAVES) {
                        self.level.set_block_at(bx, y, bz, self.feature.leaves_block).unwrap();
                    }
                }
            }
        }

    }

    /// Generate the main trunk for this tree.
    fn generate_trunk(&mut self, x: i32, y: i32, z: i32, height: u16) {
        let from = [x, y, z];
        let to = [x, y + height as i32, z];
        self.generate_block_line(from, to, self.feature.log_block);
    }

    /// Generate additional branches to connect nodes to trunk.
    fn generate_leaves_branches(&mut self, nodes: &Vec<(i32, i32, i32, i32)>, base_height: u16) {
        let base_height = base_height as f64;
        for &(x, y, z, y_start) in nodes {
            if (y_start - y) as f64 >= base_height {
                let from = [x, y_start, z];
                let to = [x, y, z];
                self.generate_block_line(from, to, self.feature.log_block);
            }
        }
    }

    /// Trace a given line using given block state.
    fn generate_block_line(&mut self, from: [i32; 3], to: [i32; 3], state: &'static BlockState) {
        for (x, y, z, _) in BlockLineIter::new(from, to, 0.5) {
            self.level.set_block_at(x, y, z, state).unwrap();
        }
    }

    /// Trace a ray from a given position to another, retuning the distance to the first block
    /// that is nor air nor leaves. It returns `-1` if the two points are the same or if the
    /// ray don't hit any block.
    fn count_block_line(&self, from: [i32; 3], to: [i32; 3]) -> i32 {
        for (x, y, z, step) in BlockLineIter::new(from, to, 0.0) {
            let block = self.level.get_block_at(x, y, z).unwrap().get_block();
            if block != &AIR && !self.env.blocks.has_block_tag(block, &TAG_LEAVES) {
                return step.abs();
            }
        }
        -1
        /*let mut diffs = [0; 3];
        let mut max_diff = 0i32;
        let mut max_diff_idx = 0;

        for i in 0..3 {
            let diff = to[i] - from[i];
            diffs[i] = diff;
            if diff.abs() > max_diff.abs() {
                max_diff = diff;
                max_diff_idx = i;
            }
        }

        if max_diff == 0 {
            return -1;
        }

        let a = BIG_TREE_COORD_PAIRS[max_diff_idx];
        let b = BIG_TREE_COORD_PAIRS[max_diff_idx + 3];
        let c = if max_diff > 0 { 1 } else { -1 };

        let a_factor = diffs[a] as f64 / max_diff as f64;
        let b_factor = diffs[b] as f64 / max_diff as f64;

        let mut step = 0;
        let step_limit = max_diff + c;

        let mut coords = [0; 3];

        while step != step_limit {

            coords[max_diff_idx] = from[max_diff_idx] + step;
            coords[a] = (from[a] as f64 + (step as f64 * a_factor)).floor() as i32;
            coords[b] = (from[b] as f64 + (step as f64 * b_factor)).floor() as i32;

            let block = self.level.get_block_at(coords[0], coords[1], coords[2]).unwrap().get_block();

            if block != &AIR && !self.env.blocks.has_block_tag(block, &TAG_LEAVES) {
                break;
            }

            step += c;

        }

        if step == step_limit {
            -1
        } else {
            step.abs()
        }*/
    }

    fn calc_layer_size(leaf_offset: i32, base_height: u16) -> f32 {

        if (leaf_offset as f64) < (base_height as f64 * 0.29999999999999999) {
            return -1.618;
        }

        let a = base_height as f32 / 2.0;
        let b = a - leaf_offset as f32;

        (if b == 0.0 {
            a
        } else if b.abs() >= a {
            0.0
        } else {
            (a.abs().powi(2) - b.abs().powi(2)).sqrt()
        }) * 0.5

    }

}


struct BlockLineIter {
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

        let secondary_idx = BIG_TREE_COORD_PAIRS[primary_idx];
        let tertiary_idx = BIG_TREE_COORD_PAIRS[primary_idx + 3];
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



fn fill_falling_vines(level: &mut dyn LevelView, x: i32, mut y: i32, z: i32, state: &'static BlockState) {
    for _ in 0..5 {
        level.set_block_at(x, y, z, state).unwrap();
        y -= 1;
        if !level.get_block_at(x, y, z).unwrap().is_block(&AIR) {
            break;
        }
    }
}
