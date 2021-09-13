use super::{LayerRand, Layer, LayerData, ComputeLayer, State};
use mc_vanilla::biome::{PLAINS, SNOWY_TUNDRA};

/// A layer that replace 20% of the plains with snowy tundras (ice plains).
pub struct AddSnowLayer {
    rand: LayerRand
}

impl AddSnowLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for AddSnowLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, parents: &mut [&mut dyn ComputeLayer]) {

        let input = parents[0].generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                let mut biome = input.get(dx + 1, dz + 1).expect_biome();

                if biome == &PLAINS {
                    self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
                    if self.rand.next_int(5) == 0 {
                        biome = &SNOWY_TUNDRA;
                    }
                }

                output.set(dx, dz, State::Biome(biome));

            }
        }

    }

}
