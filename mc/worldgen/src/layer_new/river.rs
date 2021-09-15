use super::{LayerRand, Layer, LayerData, State, LayerContext};
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

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        ctx.borrow_parent(0).unwrap().generate(x, z, output);

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


/// A layer that tries to connect rivers, for example if
pub struct AddRiverLayer;

impl Layer for AddRiverLayer {

    fn seed(&mut self, _seed: i64) { }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        let input = ctx.borrow_parent(0).unwrap()
            .generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                let south = input.get(dx + 0, dz + 1);
                let north = input.get(dx + 2, dz + 1);
                let west = input.get(dx + 1, dz + 0);
                let east = input.get(dx + 1, dz + 2);
                let center = input.get(dx + 1, dz + 1);

                fn is_no_river(state: State) -> bool {
                    matches!(state, State::NoRiver)
                }

                if is_no_river(center) ||
                    is_no_river(south) ||
                    is_no_river(north) ||
                    is_no_river(west) ||
                    is_no_river(east)
                {
                    output.set(dx, dz, State::River);
                } else if center != south || center != west || center != north || center != east {
                    output.set(dx, dz, State::River);
                } else {
                    output.set(dx, dz, State::NoRiver);
                }

            }
        }

    }

}
