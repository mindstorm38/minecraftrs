use super::{LayerRand, Layer, LayerData, ComputeLayer, State};
use mc_vanilla::biome::OCEAN;


/// A biome layer that will replace all biomes by `State::NoRiver` if the biome is an `OCEAN`
/// or by `State::PotentialRiver` with a random value in range `[2;4[`.
/// This layer is commonly used in parallel with actual biomes generation in is combined
/// with this other layer by a custom layer that take the biome layer and this river layer.
pub struct InitRiverLayer {
    rand: LayerRand
}

impl InitRiverLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for InitRiverLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, parents: &mut [&mut dyn ComputeLayer]) {

        parents[0].generate(x, z, output);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {
                self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
                let state = output.get_mut(dx, dz);
                *state = if state.expect_biome() == &OCEAN {
                    State::NoRiver
                } else {
                    State::PotentialRiver(self.rand.next_int(2) as u8 + 2)
                };
            }
        }

    }

}
