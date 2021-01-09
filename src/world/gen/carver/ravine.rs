use crate::world::chunk::Chunk;
use crate::math::{JAVA_PI, mc_sin, mc_cos};
use super::CarverInternal;


fn gen_ravine(ccx: i32, ccz: i32, chunk: &mut Chunk, internal: &mut CarverInternal) {

    let rand = &mut internal.rand;

    if rand.next_int_bounded(50) == 0 {

        let x = ccx * 16 + rand.next_int_bounded(16);
        let y = rand.next_int_bounded(40) + 8;
        let y = rand.next_int_bounded(y) + 20;
        let z = ccz * 16 + rand.next_int_bounded(16);

        // println!("[{}/{}] Generate ravine at {}/{}/{}", chunk.get_position().0, chunk.get_position().1, x, y, z);

        // Yaw: Around Y, Pitch: Around the horizontal line
        let angle_yaw = rand.next_float() * JAVA_PI as f32 * 2.0;
        let angle_pitch = ((rand.next_float() - 0.5) * 2.0) / 8.0;
        let a = (rand.next_float() * 2.0 + rand.next_float()) * 2.0;

        let new_seed = rand.next_long();
        rand.set_seed(new_seed);

        gen_ravine_worker(chunk, x as f64, y as f64, z as f64, a, angle_yaw, angle_pitch, 0, 0, 3.0, internal);

    }

}

fn gen_ravine_worker(
    chunk: &mut Chunk,
    mut x: f64,
    mut y: f64,
    mut z: f64,
    param_a: f32, // Maybe the width
    mut angle_yaw: f32,
    mut angle_pitch: f32,
    mut offset: i32,
    mut length: i32,
    param_b: f64, // Maybe the height/width
    internal: &mut CarverInternal)
{

    let (cx, cz) = chunk.get_position();
    let x_mid_chunk = (cx * 16 + 8) as f64;
    let z_mid_chunk = (cz * 16 + 8) as f64;

    //let debug = cx == -21 && cz == 34 && x == -306.0 && y == 65.0 && z == 631.0;
    let debug = false;

    if debug {
        //println!(" => x: {}, y: {}, z: {}", x, y, z);
        //println!(" => yaw: {}, pitch: {}, width: {}, offset: {}, length: {}", angle_yaw, angle_pitch, param_a, offset, length);
        //println!(" => mid chunk: {}/{}", x_mid_chunk, z_mid_chunk);
    }

    //println!(" => mid chunk: {}/{}", x_mid_chunk, z_mid_chunk);

    let mut yaw_modifier = 0.0;
    let mut pitch_modifier = 0.0;

    if length <= 0 {
        let i = internal.range * 16 - 16;
        length = i - internal.rand.next_int_bounded(i / 4);
    }

    let mut flag = false;

    if offset < 0 {
        offset = length / 2;
        flag = true;
    }

    if debug {
        //println!(" => offset: {}, length: {}", offset, length);
    }

    let table = {
        let mut table = [0f32; 128];
        let mut table_val = 1.0;
        for i in 0..128 {
            if i == 0 || internal.rand.next_int_bounded(3) == 0 {
                table_val = 1.0 + internal.rand.next_float() * internal.rand.next_float() * 1.0;
            }
            table[i] = table_val * table_val;
        }
        table
    };

    'length_loop: for offset in offset..length {

        if debug {
            //println!(" => loop {}/{}", offset, length);
        }

        let mut a = 1.5 + (mc_sin(offset as f32 * JAVA_PI as f32 / length as f32) * param_a * 1.0) as f64;
        let mut b = a * param_b;

        a *= internal.rand.next_float() as f64 * 0.25 + 0.75;
        b *= internal.rand.next_float() as f64 * 0.25 + 0.75;

        if debug {
            //println!(" => a: {}, b: {}", a, b);
        }

        let pitch_cos = mc_cos(angle_pitch);
        let pitch_sin = mc_sin(angle_pitch);

        x += (mc_cos(angle_yaw) * pitch_cos) as f64;
        y += pitch_sin as f64;
        z += (mc_sin(angle_yaw) * pitch_cos) as f64;

        if debug {
            //println!(" => x: {}, y: {}, z: {}", x, y, z);
        }

        angle_pitch *= 0.7;
        angle_pitch += pitch_modifier * 0.05;
        angle_yaw += yaw_modifier * 0.05;

        pitch_modifier *= 0.8;
        yaw_modifier *= 0.5;

        pitch_modifier += (internal.rand.next_float() - internal.rand.next_float()) * internal.rand.next_float() * 2.0;
        yaw_modifier += (internal.rand.next_float() - internal.rand.next_float()) * internal.rand.next_float() * 4.0;

        if !flag && internal.rand.next_int_bounded(4) == 0 {
            /*if debug {
                println!(" => skip this")
            }*/
            continue;
        }

        let x_chunk_rel = x - x_mid_chunk;
        let z_chunk_rel = z - z_mid_chunk;
        let remaining_length = (length - offset) as f64;
        let c = (param_a + 2.0 + 16.0) as f64;

        if debug {
            //println!(" => chunk rel: {}/{}, remaining length: {}, c: {}", x_chunk_rel, z_chunk_rel, remaining_length, c);
        }

        if x_chunk_rel * x_chunk_rel + z_chunk_rel * z_chunk_rel - remaining_length * remaining_length > c * c {
            break;
        }

        if x < x_mid_chunk - 16.0 - a * 2.0 || z < z_mid_chunk - 16.0 - a * 2.0 || x > x_mid_chunk + 16.0 + a * 2.0 || z > z_mid_chunk + 16.0 + a * 2.0 {
            /*if debug {
                println!(" => skip this (2)")
            }*/
            continue;
        }

        let x_start = ((x - a).floor() as i32 - cx * 16 - 1).max(0) as usize;
        let x_end = ((x + a).floor() as i32 - cx * 16 + 1).max(0).min(16) as usize;
        let y_start = ((y - b).floor() as i32 - 1).max(1) as usize;
        let y_end = ((y + b).floor() as i32 + 1).max(0).min(120) as usize;
        let z_start = ((z - a).floor() as i32 - cz * 16 - 1).max(0) as usize;
        let z_end = ((z + a).floor() as i32 - cz * 16 + 1).max(0).min(16) as usize;

        if debug {
            //println!(" => x: [{};{}[, y: [{};{}[, z: [{};{}[", x_start, x_end, y_start, y_end, z_start, z_end);
        }

        for bx in x_start..x_end {
            for bz in z_start..z_end {
                let mut by = y_end + 1;
                while by >= y_start - 1 {
                    if by >= 0 && by < 128 {

                        // Replace with water id from registry
                        let must_stop = chunk.get_block_id(bx, by, bz) == 9;

                        if by != y_start - 1 && bx != x_start && bx != x_end - 1 && bz != z_start && bz != z_end - 1 {
                            by = y_start;
                        }

                        if must_stop {
                            /*if debug {
                                println!(" => skip this (3)")
                            }*/
                            continue'length_loop;
                        }

                        by -= 1;

                    }
                }
            }
        }

        for bx in x_start..x_end {

            let dx = ((bx as i32 + cx * 16) as f64 + 0.5 - x) / a;

            for bz in z_start..z_end {

                let dz = ((bz as i32 + cz * 16) as f64 + 0.5 - z) / a;

                if dx * dx + dz * dz >= 1.0 {
                    continue;
                }

                let mut by = y_end - 1;
                let mut pierced_ground = false;

                while by >= y_start {

                    let dy = (by as f64 + 0.5 - y) / b;

                    if (dx * dx + dz * dz) * table[by] as f64 + ((dy * dy) / 6.0) < 1.0 {

                        // Why: in Minecraft code the "by" value has an
                        // offset of 1 block to the bottom.
                        let rby = by + 1;

                        let block_id = chunk.get_block_id(bx, rby, bz);

                        if block_id == 2 { // Replace with 'grass' id
                            pierced_ground = true;
                        }

                        if block_id == 1 || block_id == 3 || block_id == 2 {
                            if by < 10 {
                                chunk.set_block_id(bx, rby, bz, 11); // Lava block
                            } else {
                                chunk.set_block_id(bx, rby, bz, 0);
                                if pierced_ground && chunk.get_block_id(bx, rby - 1, bz) == 3 { // If dirt
                                    chunk.set_block_id(bx, rby - 1, bz, 2); // TODO Replace the 2 with query to biome generation.
                                }
                            }
                        }

                    }

                    by -= 1;

                }

            }
        }

        if flag {
            break;
        }

    }

}

impl_carver!(gen_ravine, new_ravine, 8);
