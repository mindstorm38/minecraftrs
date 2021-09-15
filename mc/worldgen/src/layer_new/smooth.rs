use super::{Layer, LayerRand, LayerData, LayerContext};


/// This layer smooth the input layer by removed micro biomes and filling
/// gaps between cells of the same biome.
pub struct SmoothLayer {
    rand: LayerRand
}

impl SmoothLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for SmoothLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        let input = ctx.borrow_parent(0).unwrap()
            .generate_size(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

        for dz in 0..output.z_size {
            for dx in 0..output.x_size {

                let south = input.get(dx + 0, dz + 1);
                let north = input.get(dx + 2, dz + 1);
                let west = input.get(dx + 1, dz + 0);
                let east = input.get(dx + 1, dz + 2);
                let mut center = input.get(dx + 1, dz + 1);

                if south == north && west == east {
                    self.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
                    center = if self.rand.next_int(2) == 0 {
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

    }

}
