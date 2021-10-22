use mc_core::rand::JavaRandom;
use mc_vanilla::block::*;

use crate::feature::{Feature, LevelView};


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

        for dx in x_start..=x_end {
            for dz in z_start..=z_end {
                for dy in y_start..=y_end {
                    if dy == y_start || dy == y_end && level.get_block_at(dx, dy, dz).unwrap() == block_air /* TODO: is not solid */ {
                        return false;
                    } else if (dx == x_start || dx == x_end || dz == z_start || dz == z_end) && dy == y &&
                        level.get_block_at(dx, dy, dz).unwrap() == block_air &&
                        level.get_block_at(dx, dy + 1, dz).unwrap() == block_air {
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
                for dy in (y_start..=y_end).rev() {
                    if dx == x_start || dx == x_end || dz == z_start || dz == z_end || dy == y_start || dy == y_end {
                        if dy >= 0 && level.get_block_at(dx, dy - 1, dz).unwrap() == block_air /* TODO: is not solid */ {
                            level.set_block_at(dx, dy, dz, block_air).unwrap();
                        } else if level.get_block_at(dx, dy, dz).unwrap() != block_air /* TODO: is solid */ {
                            level.set_block_at(dx, dy, dz, if dy == y_start && rand.next_int_bounded(4) != 0 {
                                block_mossy_cobblestone
                            } else {
                                block_cobblestone
                            }).unwrap();
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
