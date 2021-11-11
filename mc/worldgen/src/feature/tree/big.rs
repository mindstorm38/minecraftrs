use std::sync::Arc;

use mc_core::world::level::LevelEnv;
use mc_core::block::BlockState;
use mc_core::rand::JavaRandom;
use mc_core::math::JAVA_PI;

use mc_vanilla::block::material::TAG_LEAVES;
use mc_vanilla::block::*;

use super::{TreePalette, TreeHeight, RandomTreeHeight, BlockLineIter};
use crate::feature::Feature;
use crate::view::LevelView;


/// Big tree feature.
///
/// Note that this feature intentionally fix the issue in old MC versions were big trees were
/// inconsistent.
pub struct BigTreeFeature<H> {
    palette: TreePalette,
    height: H,
    height_attenuation: f64,
    branch_density: f64,
    branch_slope: f64,
    scale_width: f64,
    leaf_density: f64,
    leaf_dist_limit: u16
}

impl BigTreeFeature<RandomTreeHeight> {
    pub fn new() -> Self {
        Self {
            palette: TreePalette::new_oak(),
            height: RandomTreeHeight::new(5, 12),
            height_attenuation: 0.61799999999999999,
            branch_density: 1.0,
            branch_slope: 0.38100000000000001,
            scale_width: 1.0,
            leaf_density: 1.0,
            leaf_dist_limit: 5
        }
    }
}

impl<H: TreeHeight> Feature for BigTreeFeature<H> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        BigTreeBuilder {
            rand: JavaRandom::new(rand.next_long()),
            feature: self,
            env: Arc::clone(level.get_env()),
            level
        }.generate(x, y, z)
    }
}


/// Internal temporary builder structure for big tree.
struct BigTreeBuilder<'a, 'b, H> {
    rand: JavaRandom,
    feature: &'a BigTreeFeature<H>,
    env: Arc<LevelEnv>,
    level: &'b mut dyn LevelView
}

impl<'a, 'b, H: TreeHeight> BigTreeBuilder<'a, 'b, H> {

    fn generate(mut self, x: i32, y: i32, z: i32) -> bool {

        let base_height = match self.is_valid_position(x, y, z) {
            Ok(v) => v,
            _ => return false
        };

        let (height, leaf_nodes) = self.generate_leaf_nodes(x, y, z, base_height);

        self.generate_leaves(&leaf_nodes);
        self.generate_trunk(x, y, z, height);
        self.generate_leaves_branches(x, y, z, base_height, &leaf_nodes);
        true

    }

    /// Check if the base position of the trunk would allow a big tree to generate,
    /// returning `Err(())` if it is impossible or `Ok(base_height)`.
    fn is_valid_position(&mut self, x: i32, y: i32, z: i32) -> Result<i32, ()> {

        let ground_block = self.level.get_block_at(x, y - 1, z).unwrap().get_block();
        if ground_block != &GRASS_BLOCK && ground_block != &DIRT {
            return Err(());
        }

        let base_height = self.feature.height.gen(&mut self.rand);

        let trunk_from = [x, y, z];
        let trunk_to = [x, y + base_height, z];
        let trunk_length = self.count_block_line(trunk_from, trunk_to);

        if trunk_length == -1 {
            Ok(base_height)
        } else if trunk_length < 6 {
            Err(())
        } else {
            Ok(trunk_length)
        }

    }

    fn generate_leaf_nodes(&mut self, x: i32, y: i32, z: i32, base_height: i32) -> (i32, Vec<(i32, i32, i32, i32)>) {

        let mut height = (base_height as f64 * self.feature.height_attenuation) as i32;
        if height >= base_height {
            height = base_height - 1;
        }

        let a = ((1.3819999999999999 + ((self.feature.leaf_density * base_height as f64) / 13.0).powf(2.0)) as i32).max(1) as usize;

        let mut leaf_nodes = Vec::with_capacity(a * base_height as usize);
        let mut leaf_offset = base_height - self.feature.leaf_dist_limit as i32;
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

                        let branch_from = [dx, branch_dy, dz];
                        let branch_to = [dx, dy, dz];

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
        let leaves_block = self.feature.palette.get_leaves(1);

        for dx in -radius_int..=radius_int {
            for dz in -radius_int..=radius_int {
                let dist = (((dx as f64).abs() + 0.5).powf(2.0) + ((dz as f64).abs() + 0.5).powf(2.0)).sqrt();
                if dist <= radius_f64 {
                    let bx = x + dx;
                    let bz = z + dz;
                    let block = self.level.get_block_at(bx, y, bz).unwrap().get_block();
                    if block == &AIR || self.env.blocks.has_block_tag(block, &TAG_LEAVES) {
                        self.level.set_block_at(bx, y, bz, leaves_block).unwrap();
                    }
                }
            }
        }

    }

    /// Generate the main trunk for this tree.
    fn generate_trunk(&mut self, x: i32, y: i32, z: i32, height: i32) {
        let from = [x, y, z];
        let to = [x, y + height, z];
        self.generate_block_line(from, to, self.feature.palette.log);
    }

    /// Generate additional branches to connect nodes to trunk.
    fn generate_leaves_branches(&mut self, x: i32, y: i32, z: i32, base_height: i32, nodes: &Vec<(i32, i32, i32, i32)>) {
        let min_height = base_height as f64 * 0.20000000000000001;
        for &(nx, ny, nz, ty) in nodes {
            if (ty - y) as f64 >= min_height {
                let from = [x, ty, z];
                let to = [nx, ny, nz];
                self.generate_block_line(from, to, self.feature.palette.log);
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
    }

    fn calc_layer_size(leaf_offset: i32, base_height: i32) -> f32 {

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
