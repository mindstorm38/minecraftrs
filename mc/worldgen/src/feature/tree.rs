use std::sync::Arc;

use mc_core::block::{Block, BlockState};
use mc_core::rand::JavaRandom;
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


fn fill_falling_vines(level: &mut dyn LevelView, x: i32, mut y: i32, z: i32, state: &'static BlockState) {
    for _ in 0..5 {
        level.set_block_at(x, y, z, state).unwrap();
        y -= 1;
        if !level.get_block_at(x, y, z).unwrap().is_block(&AIR) {
            break;
        }
    }
}
