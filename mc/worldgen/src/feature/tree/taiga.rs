use std::sync::Arc;

use mc_core::rand::JavaRandom;

use mc_vanilla::block::material::TAG_LEAVES;
use mc_vanilla::block::*;

use super::{TreePalette, TreeHeight, RandomTreeHeight, generate_leaves_layer};
use crate::feature::Feature;
use crate::view::LevelView;

/// A taiga tree feature providing generation of pine and spruce trees.
/// This feature even provide a way to customize the palette.
pub struct TaigaTreeFeature<H> {
    palette: TreePalette,
    height: H,
    layered: bool
}

impl<H: TreeHeight> TaigaTreeFeature<H> {
    pub fn new(palette: TreePalette, height: H, layered: bool) -> Self {
        Self { palette, height, layered }
    }
}

impl TaigaTreeFeature<RandomTreeHeight> {

    pub fn new_spruce() -> Self {
        Self::new(TreePalette::new_spruce(), RandomTreeHeight::new(6, 4), true)
    }

    pub fn new_pine() -> Self {
        Self::new(TreePalette::new_spruce(), RandomTreeHeight::new(7, 5), false)
    }

}

impl<H: TreeHeight> Feature for TaigaTreeFeature<H> {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {

        let height = self.height.gen(rand);
        let crown_offset;
        let crown_height;
        let max_radius;
        let max_y;

        if self.layered {
            crown_offset = rand.next_int_bounded(2) + 1;
            crown_height = height - crown_offset;
            max_radius = 2 + rand.next_int_bounded(2);
            max_y = 256;
        } else {
            crown_height = rand.next_int_bounded(2) + 3;
            crown_offset = height - crown_height;
            max_radius = 1 + rand.next_int_bounded(crown_height + 1);
            max_y = 128;
        }

        if y < 1 || y + height + 1 > max_y {
            return false;
        }

        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;

        for dy in y..=(y + height + 1) {

            let radius = if dy < y + crown_offset {
                0
            } else {
                max_radius
            };

            for dx in (x - radius)..=(x + radius) {
                for dz in (z - radius)..=(z + radius) {
                    if dy >= 0 && dy < max_y {
                        let block = level.get_block_at(dx, dy, dz).unwrap().get_block();
                        if block != &AIR && !env_blocks.has_block_tag(block, &TAG_LEAVES) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }

        }

        let block_air = AIR.get_default_state();
        let ground_block = level.get_block_at(x, y - 1, z).unwrap_or(block_air).get_block();
        if (ground_block != &GRASS_BLOCK && ground_block != &DIRT) || y >= max_y - height - 1 {
            return false;
        }

        level.set_block_at(x, y - 1, z, DIRT.get_default_state()).unwrap();

        // Blocks palette
        let block_leaves = self.palette.get_leaves(1);
        let block_log = self.palette.log;

        let mut radius = if self.layered {
            rand.next_int_bounded(2)
        } else {
            0
        };

        // Only used for layered spruces.
        let mut current_max_radius = 1;
        let mut secondary_crown = false;

        for dy in ((y + crown_offset)..=(y + height)).rev() {

            generate_leaves_layer(level, env_blocks, x, dy, z, radius, block_leaves, move || radius <= 0);

            if self.layered {
                if radius >= current_max_radius {
                    radius = secondary_crown as i32;
                    secondary_crown = true;
                    current_max_radius += 1;
                    if current_max_radius > max_radius {
                        current_max_radius = max_radius;
                    }
                } else {
                    radius += 1;
                }
            } else {
                if radius >= 1 && dy == y + crown_offset + 1 {
                    radius -= 1;
                } else if radius < max_radius {
                    radius += 1;
                }
            }

        }

        let log_height_offset = if self.layered {
            rand.next_int_bounded(3)
        } else {
            1
        };

        for dy in y..(y + height - log_height_offset) {
            let block = level.get_block_at(x, dy, z).unwrap().get_block();
            if block == &AIR || env_blocks.has_block_tag(block, &TAG_LEAVES) {
                level.set_block_at(x, dy, z, block_log).unwrap();
            }
        }

        true

    }

}
