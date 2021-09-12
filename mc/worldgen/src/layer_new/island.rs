use crate::layer_new::{LayerData, ComputeLayer, LayerRand, State};
use super::Layer;

use mc_vanilla::biome::{PLAINS, OCEAN};
use mc_core::biome::Biome;


/// The initial island layer, it set the biome to either `PLAINS` or `OCEAN`.
pub struct IslandLayer {
    rand: LayerRand
}

impl IslandLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for IslandLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, _parents: &[&mut dyn ComputeLayer]) {

        for dz in 0..output.x_size {
            for dx in 0..output.z_size {
                self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
                output.set(dx, dz, match self.rand.next_int(10) {
                    0 => State::Biome(&PLAINS),
                    _ => State::Biome(&OCEAN)
                })
            }
        }

        if x <= 0 && z <= 0 && x > -(output.x_size as i32) && z > -(output.z_size as i32) {
            output.set((-x) as usize, (-z) as usize, State::Biome(&PLAINS));
        }

    }

}

/// A layer to add island to the parent biomes map.
pub struct AddIslandLayer {
    rand: LayerRand,
}

impl AddIslandLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for AddIslandLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, parents: &[&mut dyn ComputeLayer]) {

        macro_rules! post_inc {
            ($v:ident) => (($v, $v += 1).0);
        }

        #[inline]
        fn is_ocean(biome: &'static Biome) -> bool {
            biome == &OCEAN
        }

        let input = parents[0].generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                let sw = input.get(dx + 0, dz + 0).expect_biome();
                let nw = input.get(dx + 2, dz + 0).expect_biome();
                let se = input.get(dx + 0, dz + 2).expect_biome();
                let ne = input.get(dx + 2, dz + 2).expect_biome();
                let mut center = input.get(dx + 1, dz + 1).expect_biome();

                self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);

                if is_ocean(center) && (!is_ocean(sw) || !is_ocean(nw) || !is_ocean(se) || !is_ocean(ne)) {

                    let mut bound = 1;
                    let mut to_set = &PLAINS;

                    if !is_ocean(sw) && self.rand.next_int(post_inc!(bound)) == 0 {
                        to_set = sw;
                    }

                    if !is_ocean(nw) && self.rand.next_int(post_inc!(bound)) == 0 {
                        to_set = nw;
                    }

                    if !is_ocean(se) && self.rand.next_int(post_inc!(bound)) == 0 {
                        to_set = se;
                    }

                    if !is_ocean(ne) && self.rand.next_int(bound) == 0 {
                        to_set = ne;
                    }

                    center = if self.rand.next_int(3) == 0 {
                        to_set
                    } else if to_set == ICE_PLAINS::ID {
                        FROZEN_OCEAN::ID
                    } else {
                        OCEAN::ID
                    };

                } else if !is_ocean(center) && (is_ocean(sw) || is_ocean(nw) || is_ocean(se) || is_ocean(ne)) {

                    if self.rand.next_int(5) == 0 {
                        center = match center {
                            ICE_PLAINS::ID => FROZEN_OCEAN::ID,
                            _ => OCEAN::ID
                        };
                    }

                }

                output.set(dx, dz, State::Biome(center));

            }
        }

    }

}
