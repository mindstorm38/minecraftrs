use super::{LayerData, LayerInternal, State};
use crate::biome::def::{OCEAN, PLAINS, FROZEN_OCEAN, ICE_PLAINS, MUSHROOM_ISLAND};


/// This layer initiate the output with State::Land or State::Ocean,
/// real biomes are only decided on "biome layer".
fn island(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    for dz in 0..output.x_size {
        for dx in 0..output.z_size {
            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
            output.set(dx, dz, match internal.rand.next_int(10) {
                0 => State::Biome(PLAINS::ID),
                _ => State::Biome(OCEAN::ID)
            })
        }
    }

    if x <= 0 && z <= 0 && x > -(output.x_size as i32) && z > -(output.z_size as i32) {
        output.set((-x) as usize, (-z) as usize, State::Biome(PLAINS::ID));
    }

    output.debug("island");

}


#[inline]
const fn is_ocean(biome: u8) -> bool {
    biome == OCEAN::ID
}


macro_rules! post_inc {
    ($v:ident) => (($v, $v += 1).0);
}


/// This layer adds islands or ocean.
fn add_island(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            let sw = input.get(dx + 0, dz + 0).expect_biome();
            let nw = input.get(dx + 2, dz + 0).expect_biome();
            let se = input.get(dx + 0, dz + 2).expect_biome();
            let ne = input.get(dx + 2, dz + 2).expect_biome();
            let mut center = input.get(dx + 1, dz + 1).expect_biome();

            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

            if is_ocean(center) && (!is_ocean(sw) || !is_ocean(nw) || !is_ocean(se) || !is_ocean(ne)) {

                let mut bound = 1;
                let mut to_set = PLAINS::ID;

                if !is_ocean(sw) && internal.rand.next_int(post_inc!(bound)) == 0 {
                    to_set = sw;
                }

                if !is_ocean(nw) && internal.rand.next_int(post_inc!(bound)) == 0 {
                    to_set = nw;
                }

                if !is_ocean(se) && internal.rand.next_int(post_inc!(bound)) == 0 {
                    to_set = se;
                }

                if !is_ocean(ne) && internal.rand.next_int(bound/*post_inc!(bound)*/) == 0 {
                    to_set = ne;
                }

                center = if internal.rand.next_int(3) == 0 {
                    to_set
                } else if to_set == ICE_PLAINS::ID {
                    FROZEN_OCEAN::ID
                } else {
                    OCEAN::ID
                };

            } else if !is_ocean(center) && (is_ocean(sw) || is_ocean(nw) || is_ocean(se) || is_ocean(ne)) {

                if internal.rand.next_int(5) == 0 {
                    center = match center {
                        ICE_PLAINS::ID => FROZEN_OCEAN::ID,
                        _ => OCEAN::ID
                    };
                }

            }

            output.set(dx, dz, State::Biome(center));

        }
    }

    output.debug("add_island");

}

fn add_mushroom_island(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            let sw = input.get(dx + 0, dz + 0).expect_biome();
            let nw = input.get(dx + 2, dz + 0).expect_biome();
            let se = input.get(dx + 0, dz + 2).expect_biome();
            let ne = input.get(dx + 2, dz + 2).expect_biome();
            let center = input.get(dx + 1, dz + 1).expect_biome();

            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

            if is_ocean(center) && is_ocean(sw) && is_ocean(nw) && is_ocean(se) && is_ocean(ne) && internal.rand.next_int(100) == 0 {
                output.set(dx, dz, State::Biome(MUSHROOM_ISLAND::ID));
            } else {
                output.set(dx, dz, State::Biome(center));
            }

        }
    }

    output.debug("add_mushroom_island");

}

impl_layer!(orphan island, new_island);
impl_layer!(add_island, new_add_island);
impl_layer!(add_mushroom_island, new_add_mushroom_island);
