use super::{LayerData, LayerInternal};

fn smooth(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            let south = input.get(dx + 0, dz + 1);
            let north = input.get(dx + 2, dz + 1);
            let west = input.get(dx + 1, dz + 0);
            let east = input.get(dx + 1, dz + 2);
            let mut center = input.get(dx + 1, dz + 1);

            if south == north && west == east {
                internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
                center = if internal.rand.next_int(2) == 0 {
                    south
                } else {
                    west
                };
            } else if west == east {
                center = west;
            } else if south == north {
                center = south;
            }

            output.set(dx, dz, center);

        }
    }

    output.debug("smooth");

}

impl_layer!(smooth, new_smooth);
