use mc_core::world::chunk::ChunkGuard;
use mc_core::rand::JavaRandom;
use mc_core::block::BlockState;
use mc_vanilla::block::*;

use super::Feature;


pub struct GenWaterCircle {
    block: &'static BlockState,
    radius: u16
}

impl GenWaterCircle {
    pub fn new(block: &'static BlockState, radius: u16) -> Self {
        Self {
            block,
            radius
        }
    }
}

impl Feature for GenWaterCircle {

    fn generate(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {

        if let Some(state) = chunk.get_block_at_safe(x, y, z) {
            if state.is_block(&WATER) {

                let radius = rand.next_int_bounded(self.radius as i32 - 2) + 2;

                for bx in (x - radius).max(chunk.min_x)..=(x + radius).min(chunk.max_x - 1) {
                    for bz in (z - radius).max(chunk.min_z)..=(z + radius).min(chunk.max_z - 1) {
                        let dx = bx - x;
                        let dz = bz - z;
                        if dx * dx + dz * dz <= radius * radius {
                            for by in (y - 2)..=(y + 2) {
                                if let Ok(prev_state) = chunk.get_block_at(bx, by, bz) {
                                    if prev_state.is_block(&DIRT) || prev_state.is_block(&GRASS_BLOCK) {
                                        chunk.set_block_at(bx, by, bz, self.block).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }

            }
        }

    }

}