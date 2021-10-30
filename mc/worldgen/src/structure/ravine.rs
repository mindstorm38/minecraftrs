use mc_core::world::chunk::Chunk;
use mc_core::rand::JavaRandom;
use mc_core::block::BlockState;
use mc_core::math::{mc_cos, mc_sin, JAVA_PI};
use mc_vanilla::block::*;

use super::Structure;


pub struct RavineStructure<'a, F: FnMut(u8, u8) -> &'static BlockState> {
    pub get_biome_top_block: &'a mut F
}

impl<F> Structure for RavineStructure<'_, F>
where
    F: FnMut(u8, u8) -> &'static BlockState
{

    fn generate(&mut self, ccx: i32, ccz: i32, chunk: &mut Chunk, range: i32, rand: &mut JavaRandom) {

        if rand.next_int_bounded(50) == 0 {

            let x = ccx * 16 + rand.next_int_bounded(16);
            let y = {
                let v = rand.next_int_bounded(40);
                rand.next_int_bounded(v + 8) + 20
            };
            let z = ccz * 16 + rand.next_int_bounded(16);

            // Yaw: Around Y, Pitch: Around the horizontal line
            let angle_yaw = rand.next_float() * JAVA_PI as f32 * 2.0;
            let angle_pitch = ((rand.next_float() - 0.5) * 2.0) / 8.0;
            let base_width = (rand.next_float() * 2.0 + rand.next_float()) * 2.0;

            let new_seed = rand.next_long();
            gen_ravine_worker(new_seed, range, chunk, x as f64, y as f64, z as f64, base_width, angle_yaw, angle_pitch, 0, 0, 3.0, self.get_biome_top_block);

        }

    }

}


fn gen_ravine_worker(
    seed: i64,
    range: i32,
    chunk: &mut Chunk,
    mut x: f64,
    mut y: f64,
    mut z: f64,
    base_width: f32,
    mut angle_yaw: f32,
    mut angle_pitch: f32,
    mut offset: i32,
    mut length: i32,
    height_ratio: f64,
    get_biome_top_block: &mut impl FnMut(u8, u8) -> &'static BlockState
) {

    let mut rand = JavaRandom::new(seed);

    let (cx, cz) = chunk.get_position();
    let x_mid_chunk = (cx * 16 + 8) as f64;
    let z_mid_chunk = (cz * 16 + 8) as f64;

    let mut yaw_modifier = 0.0;
    let mut pitch_modifier = 0.0;

    if length <= 0 {
        let i = range * 16 - 16;
        length = i - rand.next_int_bounded(i / 4);
    }

    let mut auto_offset = false;

    if offset < 0 {
        offset = length / 2;
        auto_offset = true;
    }

    let table = {
        let mut table = [0f32; 128];
        let mut table_val = 1.0;
        for i in 0..128 {
            if i == 0 || rand.next_int_bounded(3) == 0 {
                table_val = 1.0 + rand.next_float() * rand.next_float() * 1.0;
            }
            table[i] = table_val * table_val;
        }
        table
    };

    /*// Querying block ids used in ravines
    let stone_block = chunk.get_world_info().block_registry.0.expect_from_name("stone").get_id();
    let grass_block = chunk.get_world_info().block_registry.0.expect_from_name("grass").get_id();
    let dirt_block = chunk.get_world_info().block_registry.0.expect_from_name("dirt").get_id();
    let water_block = chunk.get_world_info().block_registry.0.expect_from_name("water").get_id();
    let lava_block = chunk.get_world_info().block_registry.0.expect_from_name("lava").get_id();*/
    let air_block = AIR.get_default_state();
    let stone_block = STONE.get_default_state();
    let grass_block = GRASS_BLOCK.get_default_state();
    let dirt_block = DIRT.get_default_state();
    let water_block = WATER.get_default_state();
    let lava_block = LAVA.get_default_state();

    'length_loop: for offset in offset..length {

        let mut width = 1.5 + (mc_sin(offset as f32 * JAVA_PI as f32 / length as f32) * base_width * 1.0) as f64;
        let mut height = width * height_ratio;

        width *= rand.next_float() as f64 * 0.25 + 0.75;
        height *= rand.next_float() as f64 * 0.25 + 0.75;

        let pitch_cos = mc_cos(angle_pitch);
        let pitch_sin = mc_sin(angle_pitch);

        x += (mc_cos(angle_yaw) * pitch_cos) as f64;
        y += pitch_sin as f64;
        z += (mc_sin(angle_yaw) * pitch_cos) as f64;

        angle_pitch *= 0.7;
        angle_pitch += pitch_modifier * 0.05;
        angle_yaw += yaw_modifier * 0.05;
        pitch_modifier *= 0.8;
        yaw_modifier *= 0.5;

        pitch_modifier += (rand.next_float() - rand.next_float()) * rand.next_float() * 2.0;
        yaw_modifier += (rand.next_float() - rand.next_float()) * rand.next_float() * 4.0;

        if !auto_offset && rand.next_int_bounded(4) == 0 {
            continue;
        }

        let x_chunk_rel = x - x_mid_chunk;
        let z_chunk_rel = z - z_mid_chunk;
        let remaining_length = (length - offset) as f64;
        let c = (base_width + 2.0 + 16.0) as f64;

        if x_chunk_rel * x_chunk_rel + z_chunk_rel * z_chunk_rel - remaining_length * remaining_length > c * c {
            break;
        }

        if x < x_mid_chunk - 16.0 - width * 2.0 || z < z_mid_chunk - 16.0 - width * 2.0 || x > x_mid_chunk + 16.0 + width * 2.0 || z > z_mid_chunk + 16.0 + width * 2.0 {
            continue;
        }

        // TODO: We should not use clamp but only min/max, as can be seen in the decompilation.
        let x_start = ((x - width).floor() as i32 - cx * 16 - 1).clamp(0, 16) as u8;
        let x_end = ((x + width).floor() as i32 - cx * 16 + 1).clamp(0, 16) as u8;
        let y_start = ((y - height).floor() as i32 - 1).max(1);
        let y_end = ((y + height).floor() as i32 + 1).min(120);
        let z_start = ((z - width).floor() as i32 - cz * 16 - 1).clamp(0, 16) as u8;
        let z_end = ((z + width).floor() as i32 - cz * 16 + 1).clamp(0, 16) as u8;

        for bx in x_start..x_end {
            for bz in z_start..z_end {
                let mut by = y_end + 1;
                while by >= y_start - 1 {
                    if /*by >= 0 && */by < 128 {

                        if chunk.get_block(bx, by, bz).unwrap() == water_block {
                            continue'length_loop;
                        } else if by != y_start - 1 && bx != x_start && bx != x_end - 1 && bz != z_start && bz != z_end - 1 {
                            by = y_start;
                        }

                        by -= 1;

                    }
                }
            }
        }

        for bx in x_start..x_end {

            let dx = ((cx * 16 + (bx as i32)) as f64 + 0.5 - x) / width;

            for bz in z_start..z_end {

                let dz = ((cz * 16 + (bz as i32)) as f64 + 0.5 - z) / width;

                if dx * dx + dz * dz >= 1.0 {
                    continue;
                }

                let mut by = y_end - 1;
                let mut pierced_ground = false;

                while by >= y_start {

                    let dy = (by as f64 + 0.5 - y) / height;

                    if (dx * dx + dz * dz) * table[by as usize] as f64 + ((dy * dy) / 6.0) < 1.0 {

                        // Why: in Minecraft code the "by" value has an
                        // offset of 1 block to the bottom.
                        let rby = by + 1;

                        let state = chunk.get_block(bx, rby, bz).unwrap();

                        if state == grass_block {
                            pierced_ground = true;
                        }

                        if state == stone_block || state == dirt_block || state == grass_block {
                            if by < 10 {
                                chunk.set_block(bx, rby, bz, lava_block).unwrap();
                            } else {
                                chunk.set_block(bx, rby, bz, air_block).unwrap();
                                if pierced_ground && chunk.get_block(bx, by, bz).unwrap() == dirt_block {
                                    chunk.set_block(bx, by, bz, get_biome_top_block(bx, bz)).unwrap();
                                    // TODO: Set block at y=by to the biome top block.
                                }
                            }
                        }

                    }

                    by -= 1;

                }

            }
        }

        if auto_offset {
            break;
        }

    }

}
