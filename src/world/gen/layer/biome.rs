use super::{Layer, LayerData, LayerInternal, State};
use crate::biome::def::*;

fn biome(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal, allowed_biomes: &[u8]) {

    internal.expect_parent().inner_generate(x, z, output);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

            let to_set = match output.get(dx, dz).expect_biome() {
                OCEAN::ID => OCEAN::ID,
                MUSHROOM_ISLAND::ID => MUSHROOM_ISLAND::ID,
                PLAINS::ID => internal.rand.choose_copy(allowed_biomes),
                _ => ICE_PLAINS::ID
            };

            output.set(dx, dz, State::Biome(to_set));

        }
    }

}

fn biome_1_2(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {
    biome(x, z, output, internal, &[
        DESERT::ID,
        FOREST::ID,
        EXTREME_HILLS::ID,
        SWAMPLAND::ID,
        PLAINS::ID,
        TAIGA::ID,
        JUNGLE::ID
    ]);
    output.debug("biome_1_2");
}

fn hills(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

            let mut current = input.get(dx + 1, dz + 1).expect_biome();

            if internal.rand.next_int(3) == 0 {

                let repl = match current {
                    DESERT::ID => Some(DESERT_HILLS::ID),
                    FOREST::ID => Some(FOREST_HILLS::ID),
                    TAIGA::ID => Some(TAIGA_HILLS::ID),
                    PLAINS::ID => Some(FOREST::ID),
                    ICE_PLAINS::ID => Some(ICE_MOUNTAINS::ID),
                    JUNGLE::ID => Some(JUNGLE_HILLS::ID),
                    _ => None
                };

                if let Some(repl) = repl {

                    let south = input.get(dx + 0, dz + 1).expect_biome();
                    let north = input.get(dx + 2, dz + 1).expect_biome();
                    let west = input.get(dx + 1, dz + 0).expect_biome();
                    let east = input.get(dx + 1, dz + 2).expect_biome();

                    if south == current && north == current && west == current && east == current {
                        current = repl;
                    }

                }

            }

            output.set(dx, dz, State::Biome(current));

        }
    }

    output.debug("hills");

}

fn shore(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);
    // println!("shore at {}/{} size: {}x{}", x, z, output.x_size, output.z_size);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

            let south = input.get(dx + 0, dz + 1).expect_biome();
            let north = input.get(dx + 2, dz + 1).expect_biome();
            let west = input.get(dx + 1, dz + 0).expect_biome();
            let east = input.get(dx + 1, dz + 2).expect_biome();
            let mut center = input.get(dx + 1, dz + 1).expect_biome();

            if center == MUSHROOM_ISLAND::ID {
                if let (OCEAN::ID, OCEAN::ID, OCEAN::ID, OCEAN::ID) = (south, north, west, east) {
                    center = MUSHROOM_ISLAND_SHORE::ID;
                }
            } else if center != OCEAN::ID && center != RIVER::ID && center != SWAMPLAND::ID && center != EXTREME_HILLS::ID {
                if let (OCEAN::ID, OCEAN::ID, OCEAN::ID, OCEAN::ID) = (south, north, west, east) {
                    center = BEACH::ID;
                }
            } else if center == EXTREME_HILLS::ID {
                if south != EXTREME_HILLS::ID || north != EXTREME_HILLS::ID || west != EXTREME_HILLS::ID || east != EXTREME_HILLS::ID {
                    center = EXTREME_HILLS_EDGE::ID;
                }
            }

            output.set(dx, dz, State::Biome(center));

        }
    }

    output.debug("shore");

}

fn biome_rivers(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);
    // println!("biome_rivers at {}/{} size: {}x{}", x, z, output.x_size, output.z_size);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

            let mut biome = input.get(dx + 1, dz + 1).expect_biome();

            if biome == SWAMPLAND::ID && internal.rand.next_int(6) == 0 {
                biome = RIVER::ID;
            } else if (biome == JUNGLE::ID || biome == JUNGLE_HILLS::ID) && internal.rand.next_int(8) == 0 {
                biome = RIVER::ID;
            }

            output.set(dx, dz, State::Biome(biome));

        }
    }

    output.debug("biome_rivers");

}

fn mix_biome_river(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    println!("\nBIOME LAYERS :");
    internal.expect_parent().inner_generate(x, z, output);

    println!("\nRIVER LAYERS :");
    let mut rivers_it = internal.expect_parent_aux().generate(x, z, output.x_size, output.z_size).data.into_iter();

    for current in &mut output.data {

        let biome = current.expect_biome();
        let river = rivers_it.next().unwrap(); // Unwrap should never fail as the two LayerData are of the same size.

        if biome != OCEAN::ID && matches!(river, State::River) {

            let new_biome = match biome {
                ICE_PLAINS::ID => FROZEN_RIVER::ID,
                MUSHROOM_ISLAND::ID | MUSHROOM_ISLAND_SHORE::ID => MUSHROOM_ISLAND_SHORE::ID,
                _ => RIVER::ID
            };

            *current = State::Biome(new_biome);

        }

    }

    output.debug("mix_biome_river");

}

impl_layer!(biome_1_2, new_biome_1_2);
impl_layer!(hills, new_hills);
impl_layer!(shore, new_shore);
impl_layer!(biome_rivers, new_biome_rivers);

impl Layer {
    pub fn new_mix_biome_river(base_seed: i64, biome_parent: Layer, river_parent: Layer) -> Self {
        Self::new(base_seed, mix_biome_river, Some(Box::new(biome_parent)), Some(Box::new(river_parent)))
    }
}
