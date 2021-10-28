use mc_core::rand::JavaRandom;

use mc_vanilla::block::material::TAG_NON_SOLID;
use mc_vanilla::block::*;

use crate::feature::{Feature, LevelView};
use std::sync::Arc;


pub struct DungeonFeature;

impl Feature for DungeonFeature {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {

        let x_size = rand.next_int_bounded(2) + 2;
        let z_size = rand.next_int_bounded(2) + 2;
        let mut air_count = 0u16;

        let x_start = x - x_size - 1;
        let z_start = z - z_size - 1;
        let y_start = y - 1;
        let x_end = x + x_size + 1;
        let z_end = z + z_size + 1;
        let y_end = y + 4;

        let block_air = AIR.get_default_state();
        let env = Arc::clone(level.get_env());
        let env_blocks = &env.blocks;

        for dx in x_start..=x_end {
            for dy in y_start..=y_end {
                for dz in z_start..=z_end {
                    let block = level.get_block_at(dx, dy, dz).unwrap_or(block_air);
                    if (dy == y_start || dy == y_end) && env_blocks.has_block_tag(block.get_block(), &TAG_NON_SOLID) {
                        return false;
                    } else if (dx == x_start || dx == x_end || dz == z_start || dz == z_end) &&
                        dy == y &&
                        block == block_air &&
                        level.get_block_at(dx, dy + 1, dz).unwrap_or(block_air) == block_air
                    {
                        air_count += 1;
                        if air_count > 5 {
                            return false;
                        }
                    }
                }
            }
        }

        if air_count == 0 {
            return false;
        }

        let block_mossy_cobblestone = MOSSY_COBBLESTONE.get_default_state();
        let block_cobblestone = COBBLESTONE.get_default_state();

        for dx in x_start..=x_end {
            for dz in z_start..=z_end {
                for dy in (y_start..y_end).rev() {
                    if dx == x_start || dx == x_end || dz == z_start || dz == z_end || dy == y_start || dy == y_end {

                        if dy >= 0 && env_blocks.has_block_tag(level.get_block_at(dx, dy - 1, dz).unwrap().get_block(), &TAG_NON_SOLID) {
                            level.set_block_at(dx, dy, dz, block_air).unwrap();
                        } else if !env_blocks.has_block_tag(level.get_block_at(dx, dy, dz).unwrap().get_block(), &TAG_NON_SOLID) {

                            let block_to_set = if dy == y_start && rand.next_int_bounded(4) != 0 {
                                block_mossy_cobblestone
                            } else {
                                block_cobblestone
                            };

                            level.set_block_at(dx, dy, dz, block_to_set).unwrap();

                        }
                    } else {
                        level.set_block_at(dx, dy, dz, block_air).unwrap();
                    }

                }
            }
        }

        // TODO

        true

    }
}