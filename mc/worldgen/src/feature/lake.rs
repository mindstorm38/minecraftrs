use mc_core::block::BlockState;
use mc_core::rand::JavaRandom;

use mc_vanilla::block::material::{TAG_LIQUID, TAG_NON_SOLID};
use mc_vanilla::block::*;

use super::{Feature, LevelView};


pub struct LakeFeature {
    block: &'static BlockState
}

impl LakeFeature {
    pub const fn new(block: &'static BlockState) -> Self {
        Self {
            block
        }
    }
}

impl Feature for LakeFeature {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {

        // level.set_block_at(x, y, z, GOLD_BLOCK.get_default_state()).unwrap();

        let x = x - 8;
        let mut y = y;
        let z = z - 8;

        while y > 5 && level.get_block_at(x, y, z).unwrap().is_block(&AIR) {
            y -= 1;
        }

        // level.set_block_at(x, y, z, IRON_BLOCK.get_default_state()).unwrap();

        if y <= 4 {
            return false;
        } else {
            y -= 4;
        }

        let mut flags = [false; 2048];
        for _ in 0..(rand.next_int_bounded(4) + 4) {
            let a = rand.next_double() * 6.0 + 3.0;
            let b = rand.next_double() * 4.0 + 2.0;
            let c = rand.next_double() * 6.0 + 3.0;
            let aa = rand.next_double() * (16.0 - a - 2.0) + 1.0 + a / 2.0;
            let bb = rand.next_double() * (8.0 - b - 4.0) + 2.0 + b / 2.0;
            let cc = rand.next_double() * (16.0 - c - 2.0) + 1.0 + c / 2.0;
            for dx in 1..15 {
                for dz in 1..15 {
                    for dy in 1..7 {
                        let x_dist = (dx as f64 - aa) / (a / 2.0);
                        let y_dist = (dy as f64 - bb) / (b / 2.0);
                        let z_dist = (dz as f64 - cc) / (c / 2.0);
                        if x_dist * x_dist + y_dist * y_dist + z_dist * z_dist < 1.0 {
                            flags[(dx * 16 + dz) * 8 + dy] = true;
                        }
                    }
                }
            }
        }

        for dx in 0..16 {
            for dz in 0..16 {
                for dy in 0..8 {

                    let flag = !flags[(dx * 16 + dz) * 8 + dy] && (
                        (dx != 15 && flags[((dx + 1) * 16 + dz) * 8 + dy]) ||
                        (dx != 0 && flags[((dx - 1) * 16 + dz) * 8 + dy]) ||
                        (dz != 15 && flags[(dx * 16 + (dz + 1)) * 8 + dy]) ||
                        (dz != 0 && flags[(dx * 16 + (dz - 1)) * 8 + dy]) ||
                        (dy != 7 && flags[(dx * 16 + dz) * 8 + (dy + 1)]) ||
                        (dy != 0 && flags[(dx * 16 + dz) * 8 + (dy - 1)])
                    );

                    if flag {

                        let block = level.get_block_at(x + dx as i32, y + dy as i32, z + dz as i32).unwrap().get_block();

                        let env_blocks = &level.get_env().blocks;
                        if dy >= 4 && env_blocks.has_block_tag(block, &TAG_LIQUID) {
                            return false;
                        } else  if dy < 4 && env_blocks.has_block_tag(block, &TAG_NON_SOLID) && block != self.block.get_block() {
                            return false;
                        }

                    }

                }
            }
        }

        let block_air = AIR.get_default_state();

        for dx in 0..16 {
            for dz in 0..16 {
                for dy in 0..8 {
                    if flags[(dx * 16 + dz) * 8 + dy] {
                        level.set_block_at(x + dx as i32, y + dy as i32, z + dz as i32, if dy < 4 {
                            self.block
                        } else {
                            block_air
                        }).unwrap();
                    }
                }
            }
        }

        // TODO: Finish mycelium/grass replacement

        if self.block.is_block(&LAVA) {

            let block_stone = STONE.get_default_state();

            for dx in 0..16 {
                for dz in 0..16 {
                    for dy in 0..8 {

                        let flag = !flags[(dx * 16 + dz) * 8 + dy] && (
                            (dx != 15 && flags[((dx + 1) * 16 + dz) * 8 + dy]) ||
                            (dx != 0 && flags[((dx - 1) * 16 + dz) * 8 + dy]) ||
                            (dz != 15 && flags[(dx * 16 + (dz + 1)) * 8 + dy]) ||
                            (dz != 0 && flags[(dx * 16 + (dz - 1)) * 8 + dy]) ||
                            (dy != 7 && flags[(dx * 16 + dz) * 8 + (dy + 1)]) ||
                            (dy != 0 && flags[(dx * 16 + dz) * 8 + (dy - 1)])
                        );

                        if flag && (dy < 4 || rand.next_int_bounded(2) != 0) {
                            let env_blocks = &level.get_env().blocks;
                            let block = level.get_block_at(x + dx as i32, y + dy as i32, z + dz as i32).unwrap().get_block();
                            if !env_blocks.has_block_tag(block, &TAG_NON_SOLID) {
                                level.set_block_at(x + dx as i32, y + dy as i32, z + dz as i32, block_stone);
                            }
                        }

                    }
                }
            }

        } else if self.block.is_block(&WATER) {
            // TODO: Finish water-specific lake features (currently not
        }

        true

    }

}
