use super::{LayerData, LayerInternal, State};
use crate::biome::def::{OCEAN, PLAINS, ICE_PLAINS};

fn add_snow(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);
    println!("add_snow at {}/{} size: {}x{}", x, z, output.x_size, output.z_size);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            let biome = input.get(dx + 1, dz + 1).expect_biome();

            if biome == OCEAN::ID {
                output.set(dx, dz, State::Biome(OCEAN::ID));
            } else {

                internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

                let new_biome = match internal.rand.next_int(5) {
                    0 => ICE_PLAINS::ID,
                    _ => PLAINS::ID
                };

                output.set(dx, dz, State::Biome(new_biome));

            }

        }
    }

}

impl_layer!(add_snow, new_add_snow);
