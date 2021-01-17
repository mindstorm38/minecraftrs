use super::Feature;
use crate::world::WorldAccess;
use crate::rand::jrand::JavaRandom;
use crate::res::Registrable;

pub struct GenWaterCircle {
    block: &'static str,
    radius: u16
}

impl GenWaterCircle {

}

impl Feature for GenWaterCircle {

    fn generate(&self, world: &mut WorldAccess, rand: &mut JavaRandom, x: i32, y: i32, z: i32) {

        let water_id = world.get_info().block_registry.0.expect_from_name("water").get_id();
        let dirt_id = world.get_info().block_registry.0.expect_from_name("dirt").get_id();
        let grass_id = world.get_info().block_registry.0.expect_from_name("grass").get_id();
        let block_id = world.get_info().block_registry.0.expect_from_name(self.block).get_id();

        if world.get_block_id(x, y, z).unwrap() == water_id {

            let radius = rand.next_int_bounded(self.radius as i32 - 2) + 2;

            for bx in (x - radius)..=(x + radius) {
                for bz in (z - radius)..=(z + radius) {

                    let dx = bx - x;
                    let dz = bz - z;

                    if dx * dx + dz * dz <= radius * radius {

                        for by in (y - 2)..=(y + 2) {
                            let prev_block_id = world.get_block_id(bx, by, bz).unwrap();
                            if prev_block_id == dirt_id || prev_block_id == grass_id {
                                world.set_block_id(bx, by, bz, block_id);
                            }
                        }

                    }

                }
            }

        }

    }

}