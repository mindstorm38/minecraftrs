use crate::world::chunk::Chunk;
use crate::math::{JAVA_PI, mc_sin, mc_cos};
use super::CarverInternal;

fn gen_cave(ccx: i32, ccz: i32, chunk: &mut Chunk, internal: &mut CarverInternal) {

    let rand = &mut internal.rand;

    let count = {
        let v = rand.next_int_bounded(40);
        let v = rand.next_int_bounded(v + 1);
        rand.next_int_bounded(v + 1)
    };

    for _ in 0..count {

        let x = ccx * 16 + rand.next_int_bounded(16);
        let y = {
            let v = rand.next_int_bounded(120);
            rand.next_int_bounded(v + 8)
        };
        let z = ccz * 16 + rand.next_int_bounded(16);

        let mut normal_caves_count = 1;

        if rand.next_int_bounded(4) == 0 {
            // TODO: Generate large cave node
            normal_caves_count += rand.next_int_bounded(4);
        }

        for _ in 0..normal_caves_count {

            let angle_yaw = rand.next_float() * JAVA_PI as f32 * 2.0;
            let angle_pitch = ((rand.next_float() - 0.5) * 2.0) / 8.0;
            let mut base_width = rand.next_float() * 2.0 + rand.next_float();

            if rand.next_int_bounded(10) == 0 {
                base_width *= rand.next_float() * rand.next_float() * 3.0 + 1.0;
            }

            // TODO: Generate normal cave node

        }

    }

}

impl_carver!(gen_cave, new_cave, 8);
