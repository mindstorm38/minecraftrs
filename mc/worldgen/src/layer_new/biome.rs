use super::{Layer, LayerRand, LayerData, LayerContext, State};

use mc_core::biome::Biome;
use mc_vanilla::biome::{PLAINS, DESERT, FOREST, MOUNTAINS, SWAMP, TAIGA, JUNGLE, DESERT_HILLS, WOODED_HILLS, TAIGA_HILLS, SNOWY_TUNDRA, SNOWY_MOUNTAINS, JUNGLE_HILLS, MUSHROOM_FIELDS, OCEAN, MUSHROOM_FIELD_SHORE, RIVER, BEACH, MOUNTAIN_EDGE, FROZEN_RIVER};


/// This layer replace all incoming 'plains' biome by a random biome chosen
/// from the internal biomes slice give when constructing the layer.
pub struct BiomeLayer {
    rand: LayerRand,
    biomes: &'static [&'static Biome]
}

impl BiomeLayer {

    pub fn new(base_seed: i64, biomes: &'static [&'static Biome]) -> Self {
        Self {
            rand: LayerRand::new(base_seed),
            biomes
        }
    }

    /// Make a new biome layer for release 1.2
    pub fn new_102(base_seed: i64) -> Self {
        static BIOMES: [&'static Biome; 7] = [
            &DESERT,
            &FOREST,
            &MOUNTAINS,  // Extreme hills before 1.13
            &SWAMP,
            &PLAINS,
            &TAIGA,
            &JUNGLE
        ];
        Self::new(base_seed, &BIOMES)
    }

}

impl Layer for BiomeLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        ctx.borrow_parent(0).unwrap().generate(x, z, output);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {
                let mut biome = output.get(dx, dz).expect_biome();
                if biome == &PLAINS {
                    self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
                    output.set(dx, dz, State::Biome(self.rand.choose(self.biomes)));
                }
            }
        }

    }

}

/// This layer convert 1/3 of the non-hills biomes to their hills variant if the
/// hill is in between normal variant on the 4 sides.
pub struct HillsLayer {
    rand: LayerRand
}

impl HillsLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for HillsLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        let input = ctx.borrow_parent(0).unwrap()
            .generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

                let mut biome = input.get(dx + 1, dz + 1).expect_biome();

                if self.rand.next_int(3) == 0 {

                    let repl = match biome {
                        _ if biome == &DESERT => Some(&DESERT_HILLS),
                        _ if biome == &FOREST => Some(&WOODED_HILLS),  // Forest hills before 1.13
                        _ if biome == &TAIGA => Some(&TAIGA_HILLS),
                        _ if biome == &PLAINS => Some(&FOREST),
                        _ if biome == &SNOWY_TUNDRA => Some(&SNOWY_MOUNTAINS),
                        _ if biome == &JUNGLE => Some(&JUNGLE_HILLS),
                        _ => None
                    };

                    if let Some(repl) = repl {

                        let south = input.get(dx + 0, dz + 1).expect_biome();
                        let north = input.get(dx + 2, dz + 1).expect_biome();
                        let west = input.get(dx + 1, dz + 0).expect_biome();
                        let east = input.get(dx + 1, dz + 2).expect_biome();

                        if south == biome && north == biome && west == biome && east == biome {
                            biome = repl;
                        }

                    }

                }

                output.set(dx, dz, State::Biome(biome));

            }
        }

    }

}

/// A layer that add some shore and edge biomes depending on island and hills placements.
pub struct ShoreLayer {
    rand: LayerRand
}

impl ShoreLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for ShoreLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        let input = ctx.borrow_parent(0).unwrap()
            .generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

                let south = input.get(dx + 0, dz + 1).expect_biome();
                let north = input.get(dx + 2, dz + 1).expect_biome();
                let west = input.get(dx + 1, dz + 0).expect_biome();
                let east = input.get(dx + 1, dz + 2).expect_biome();
                let mut center = input.get(dx + 1, dz + 1).expect_biome();

                if center == &MUSHROOM_FIELDS {
                    if south == &OCEAN && north == &OCEAN && west == &OCEAN && east == &OCEAN {
                        center = &MUSHROOM_FIELD_SHORE;
                    }
                } else if center != &OCEAN && center != &RIVER && center != &SWAMP && center != &MOUNTAINS {
                    if south == &OCEAN || north == &OCEAN || west == &OCEAN || east == &OCEAN {
                        center = &BEACH;
                    }
                } else if center == &MOUNTAINS {
                    if south != &MOUNTAINS || north != &MOUNTAINS || west != &MOUNTAINS || east != &MOUNTAINS {
                        center = &MOUNTAIN_EDGE;
                    }
                }

                output.set(dx, dz, State::Biome(center));

            }
        }

    }

}


/// A layer that adds rivers in swamps and jungles.
pub struct BiomeRiversLayer {
    rand: LayerRand
}

impl BiomeRiversLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for BiomeRiversLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        let input = ctx.borrow_parent(0).unwrap()
            .generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

                let mut biome = input.get(dx + 1, dz + 1).expect_biome();

                if biome == &SWAMP && self.rand.next_int(6) == 0 {
                    biome = &RIVER;
                } else if (biome == &JUNGLE || biome == &JUNGLE_HILLS) && self.rand.next_int(8) == 0 {
                    biome = &RIVER;
                }

                output.set(dx, dz, State::Biome(biome));

            }
        }

    }

}


/// A layer that mix two parent layers, the parent #0 must be the biome layer and
/// the #1 the river layer.
pub struct MixBiomeAndRiverLayer;

impl Layer for MixBiomeAndRiverLayer {

    fn seed(&mut self, _seed: i64) { }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        ctx.borrow_parent(0).unwrap().generate(x, z, output);

        let mut rivers_it = ctx.borrow_parent(1).unwrap()
            .generate_size(x, z, output.x_size, output.z_size)
            .data.into_iter();

        for current in &mut output.data {

            let biome = current.expect_biome();
            let river = rivers_it.next().unwrap(); // Unwrap should never fail as the two LayerData are of the same size.

            if biome != &OCEAN && matches!(river, State::River) {
                *current = State::Biome(match () {
                    _ if biome == &SNOWY_TUNDRA => &FROZEN_RIVER,
                    _ if biome == &MUSHROOM_FIELDS => &MUSHROOM_FIELD_SHORE,
                    _ if biome == &MUSHROOM_FIELD_SHORE => &MUSHROOM_FIELD_SHORE,
                    _ => &RIVER
                });
            }

        }

    }

}
