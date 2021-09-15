use super::{Layer, LayerRand, State, LayerData, LayerContext};


pub struct ZoomLayer {
    rand: LayerRand,
    fuzzy: bool
}

impl ZoomLayer {

    pub fn new(base_seed: i64, fuzzy: bool) -> Self {
        Self {
            rand: LayerRand::new(base_seed),
            fuzzy
        }
    }

    #[inline]
    pub fn new_fuzzy(base_seed: i64) -> Self {
        Self::new(base_seed, true)
    }

    #[inline]
    pub fn new_smart(base_seed: i64) -> Self {
        Self::new(base_seed, false)
    }

}

impl Layer for ZoomLayer {

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext) {

        let x_half = x >> 1;
        let z_half = z >> 1;
        let x_size_half = (output.x_size >> 1) + 3;
        let z_size_half = (output.z_size >> 1) + 3;
        let x_size_rounded = x_size_half << 1;
        let z_size_rounded = z_size_half << 1;

        let input = ctx.borrow_parent(0).unwrap()
            .generate_size(x_half, z_half, x_size_half, z_size_half);

        let fuzzy = self.fuzzy;
        let mut tmp = LayerData::new(x_size_rounded, z_size_rounded, State::Uninit);

        for dz in 0..(z_size_half - 1) {

            // This move two lines by two line in 'tmp'.
            // The following instructions transform 1 point of the 'input' into 4 points in 'tmp'.

            let mut v1 = input.get(0, dz + 0);
            let mut v2 = input.get(0, dz + 1);

            for dx in 0..(x_size_half - 1) {

                self.rand.init_chunk_seed((dx as i32 + x_half) << 1, (dz as i32 + z_half) << 1);

                let v3 = input.get(dx + 1, dz + 0);
                let v4 = input.get(dx + 1, dz + 1);

                tmp.set(dx * 2 + 0, dz * 2 + 0, v1);
                tmp.set(dx * 2 + 0, dz * 2 + 1, self.rand.choose(&[v1, v2]));
                tmp.set(dx * 2 + 1, dz * 2 + 0, self.rand.choose(&[v1, v3]));
                tmp.set(dx * 2 + 1, dz * 2 + 1, if fuzzy {
                    self.rand.choose(&[v1, v3, v2, v4])
                } else {
                    choose_smart(&mut self.rand, v1, v3, v2, v4)
                });

                v1 = v3;
                v2 = v4;

            }

        }

        for dz in 0..output.z_size {
            let src_offset = (dz + (z & 1) as usize) * x_size_rounded + (x & 1) as usize;
            let dst_offset = dz * output.x_size;
            for dx in 0..output.x_size {
                output.data[dst_offset + dx] = tmp.data[src_offset + dx];
            }
        }

    }

}

/// Internal method to choose from 4 states the most commonly represented.
#[inline]
fn choose_smart(rand: &mut LayerRand, v1: State, v2: State, v3: State, v4: State) -> State {

    if v2 == v3 && v3 == v4 {
        v2
    } else if v1 == v2 && v1 == v3 {
        v1
    } else if v1 == v2 && v1 == v4 {
        v1
    } else if v1 == v3 && v1 == v4 {
        v1
    } else if v1 == v2 && v3 != v4 {
        v1
    } else if v1 == v3 && v2 != v4 {
        v1
    } else if v1 == v4 && v2 != v3 {
        v1
    } else if v2 == v1 && v3 != v4 {
        v2
    } else if v2 == v3 && v1 != v4 {
        v2
    } else if v2 == v4 && v1 != v3 {
        v2
    } else if v3 == v1 && v2 != v4 {
        v3
    } else if v3 == v2 && v1 != v4 {
        v3
    } else if v3 == v4 && v1 != v2 {
        v3
    } else if v4 == v1 && v2 != v3 {
        v3 // As in MCP 1.2.5, but weird
    } else if v4 == v2 && v1 != v3 {
        v3 // As in MCP 1.2.5, but weird
    } else if v4 == v3 && v1 != v2 {
        v3 // As in MCP 1.2.5, but weird
    } else {
        rand.choose(&[v1, v2, v3, v4])
    }

}
