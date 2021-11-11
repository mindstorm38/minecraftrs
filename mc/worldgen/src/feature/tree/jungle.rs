use std::sync::Arc;

use mc_core::block::{BlockState, GlobalBlocks};
use mc_core::math::{mc_cos, mc_sin, JAVA_PI};
use mc_core::rand::JavaRandom;

use mc_vanilla::block::material::{TAG_LEAVES, TAG_LOG, TAG_NON_SOLID, TAG_SAPLING};
use mc_vanilla::block::*;

use super::{TreePalette, TreeHeight, DoubleRandomTreeHeight, generate_leaves_layer};
use crate::feature::Feature;
use crate::view::LevelView;


/// Shrub feature for jungles.
pub struct ShrubFeature {
    palette: TreePalette
}

impl ShrubFeature {

    pub fn new(palette: TreePalette) -> Self {
        Self { palette }
    }

    pub fn new_jungle() -> Self {
        Self::new(TreePalette::new(&JUNGLE_LOG, &OAK_LEAVES))
    }

}

impl Feature for ShrubFeature {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, mut y: i32, z: i32) -> bool {

        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;

        loop {
            let block = level.get_block_at(x, y, z).unwrap().get_block();
            if (block == &AIR || env_blocks.has_block_tag(block, &TAG_LEAVES)) && y > 0 {
                y -= 1;
            } else {
                y += 1;
                break;
            }
        }

        let ground_block = level.get_block_at(x, y - 1, z).unwrap().get_block();
        if ground_block != &DIRT && ground_block != &GRASS_BLOCK {
            return false;
        }

        level.set_block_at(x, y, z, self.palette.log).unwrap();

        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;
        let block_leaves = self.palette.get_leaves(1);

        for dy in y..=(y + 2) {
            let radius = 2 - (dy - y);
            generate_leaves_layer(level, env_blocks, x, dy, z, radius, block_leaves, || {
                rand.next_int_bounded(2) != 0
            });
        }

        true

    }

}


pub struct HugeJungleTreeFeature<H> {
    palette: TreePalette,
    height: H
}

impl<H: TreeHeight> HugeJungleTreeFeature<H> {
    pub fn new(palette: TreePalette, height: H) -> Self {
        Self { palette, height }
    }
}

impl HugeJungleTreeFeature<DoubleRandomTreeHeight> {
    pub fn new_jungle() -> Self {
        Self::new(TreePalette::new_jungle(), DoubleRandomTreeHeight::new(10, 20, 3))
    }
}

impl<H: TreeHeight> Feature for HugeJungleTreeFeature<H> {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {

        let height = self.height.gen(rand);

        if y < 1 || y + height + 1 > 256 {
            return false;
        }

        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;

        for by in y..=(y + height + 1) {

            let radius = if by >= (y + height - 1) {
                2
            } else if by == y {
                1
            } else {
                2
           };

            for dx in (x - radius)..=(x + radius) {
                for dz in (z - radius)..=(z + radius) {
                    if by >= 0 && by < 256 {
                        let block = level.get_block_at(dx, by, dz).unwrap().get_block();
                        if block != &AIR && block != &GRASS_BLOCK && block != &DIRT
                            && !env_blocks.has_block_tag(block, &TAG_LEAVES)
                            && !env_blocks.has_block_tag(block, &TAG_LOG)
                            && !env_blocks.has_block_tag(block, &TAG_SAPLING)
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }

        }

        let ground_block = level.get_block_at(x, y - 1, z).unwrap().get_block();
        if (ground_block != &GRASS_BLOCK && ground_block != &DIRT) || y >= 256 - height - 1 {
            return false;
        }

        let block_dirt = DIRT.get_default_state();
        let block_leaves = self.palette.get_leaves(1);
        let block_log = self.palette.log;

        let block_vine = VINE.get_default_state();
        let block_west_vine = block_vine.with(&PROP_WEST, true).unwrap();
        let block_east_vine = block_vine.with(&PROP_EAST, true).unwrap();
        let block_south_vine = block_vine.with(&PROP_SOUTH, true).unwrap();
        let block_north_vine = block_vine.with(&PROP_NORTH, true).unwrap();

        level.set_block_at(x + 0, y - 1, z + 0, block_dirt).unwrap();
        level.set_block_at(x + 1, y - 1, z + 0, block_dirt).unwrap();
        level.set_block_at(x + 0, y - 1, z + 1, block_dirt).unwrap();
        level.set_block_at(x + 1, y - 1, z + 1, block_dirt).unwrap();
        generate_crown(level, env_blocks, x, y + height, z, 2, rand, block_leaves);

        let mut by = y + height - 2 - rand.next_int_bounded(4);
        while by > y + height / 2 {

            let angle = rand.next_float() * JAVA_PI as f32 * 2.0;
            let crown_x = x + (0.5 + mc_cos(angle) * 4.0) as i32;
            let crown_z = z + (0.5 + mc_sin(angle) * 4.0) as i32;
            generate_crown(level, env_blocks, crown_x, by, crown_z, 0, rand, block_leaves);

            for dist in 0..5 {
                let log_x = x + (1.5 + mc_cos(angle) * dist as f32) as i32;
                let log_z = z + (1.5 + mc_sin(angle) * dist as f32) as i32;
                level.set_block_at(log_x, (by - 3) + dist / 2, log_z, block_log).unwrap();
            }

            by -= 2 + rand.next_int_bounded(4);

        }

        for by in y..(y + height) {

            let dy = by - y;

            if generate_core_log(level, env_blocks, x, by, z, block_log) && dy > 0 {
                generate_core_vines(level, rand, x, by, z, -1, -1, block_east_vine, block_south_vine);
            }

            if dy < height - 1 {

                if generate_core_log(level, env_blocks, x + 1, by, z, block_log) && dy > 0 {
                    generate_core_vines(level, rand, x + 1, by, z, 1, -1, block_west_vine, block_south_vine);
                }

                if generate_core_log(level, env_blocks, x + 1, by, z + 1, block_log) && dy > 0 {
                    generate_core_vines(level, rand, x + 1, by, z + 1, 1, 1, block_west_vine, block_north_vine);
                }

                if generate_core_log(level, env_blocks, x, by, z + 1, block_log) && dy > 0 {
                    generate_core_vines(level, rand, x, by, z + 1, -1, 1, block_east_vine, block_north_vine);
                }

            }

        }

        true

    }

}


fn generate_crown(
    level: &mut dyn LevelView,
    env_blocks: &GlobalBlocks,
    x: i32, y: i32, z: i32,
    base_radius: i32,
    rand: &mut JavaRandom,
    state: &'static BlockState
) {
    for by in (y - 2)..=y {
        let dy = by - y;
        let radius = base_radius + 1 - dy;
        let radius_squared = radius * radius;
        let radius_inc_squared = (radius + 1) * (radius + 1);
        let radius_dec_squared = (radius - 1) * (radius - 1);
        for bx in (x - radius)..=(x + radius + 1) {
            let dx = bx - x;
            for bz in (z - radius)..=(z + radius + 1) {
                let dz = bz - z;
                let dist = dx * dx + dz * dz;
                if (dx >= 0 || dz >= 0 || dist <= radius_squared)
                    && ((dx <= 0 && dz <= 0) || dist <= radius_inc_squared)
                    && (rand.next_int_bounded(4) != 0 || dist <= radius_dec_squared)
                {
                    let current_block = level.get_block_at(bx, by, bz).unwrap().get_block();
                    if env_blocks.has_block_tag(current_block, &TAG_NON_SOLID) {
                        level.set_block_at(bx, by, bz, state).unwrap();
                    }
                }
            }
        }
    }
}


#[inline]
fn generate_core_log(
    level: &mut dyn LevelView,
    env_blocks: &GlobalBlocks,
    x: i32, y: i32, z: i32,
    block_log: &'static BlockState,
) -> bool {
    let current_block = level.get_block_at(x, y, z).unwrap().get_block();
    if current_block == &AIR || env_blocks.has_block_tag(current_block, &TAG_LEAVES) {
        level.set_block_at(x, y, z, block_log).unwrap();
        true
    } else {
        false
    }
}

#[inline]
fn generate_core_vines(
    level: &mut dyn LevelView,
    rand: &mut JavaRandom,
    x: i32, y: i32, z: i32,
    dx_vine: i32,
    dz_vine: i32,
    dx_vine_state: &'static BlockState,
    dz_vine_state: &'static BlockState,
) {
    if rand.next_int_bounded(3) != 0 && level.get_block_at(x + dx_vine, y, z).unwrap().is_block(&AIR) {
        level.set_block_at(x + dx_vine, y, z, dx_vine_state).unwrap();
    }
    if rand.next_int_bounded(3) != 0 && level.get_block_at(x, y, z + dz_vine).unwrap().is_block(&AIR) {
        level.set_block_at(x, y, z + dz_vine, dz_vine_state).unwrap();
    }
}
