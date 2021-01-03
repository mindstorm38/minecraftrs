use super::{LayerData, LayerInternal, State, LayerRand, Layer};

fn choose_weird(rand: &mut LayerRand, states: [State; 4]) -> State {

    let (v1, v2, v3, v4) = (states[0], states[1], states[2], states[3]);

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
    } else if v4 != v3 && v1 != v2 {
        v3 // As in MCP 1.2.5, but weird
    } else {
        rand.choose_copy(&states)
    }

}

fn common_zoom(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal, fuzzy: bool) {

    let x_half = x >> 1;
    let z_half = z >> 1;
    let x_size_half = (output.x_size >> 1) + 3;
    let z_size_half = (output.z_size >> 1) + 3;

    let input = internal.expect_parent().generate(x_half, z_half, x_size_half, z_size_half);

    // Try to replace this with LayerData
    let mut temp: Vec<State> = vec![State::Uninit; x_size_half * 2 * z_size_half * 2];

    let x_size_rounded = x_size_half << 1;

    for dz in 0..(z_size_half - 1) {

        let mut temp_index = (dz << 1) * x_size_rounded;

        let mut v1 = input.get(0, dz);
        let mut v2 = input.get(0, dz + 1);

        for dx in 0..(x_size_half - 1) {

            internal.rand.init_chunk_seed(dx as i32 + (x_half << 1), dz as i32 + (z_half << 1));

            let v3 = input.get(dx + 1, dz);
            let v4 = input.get(dx + 1, dz + 1);

            temp.insert(temp_index, v1);

            temp.insert(temp_index + x_size_rounded, internal.rand.choose_copy(&[v1, v2]));
            temp_index += 1;

            temp.insert(temp_index, internal.rand.choose_copy(&[v1, v3]));

            let to_insert = if fuzzy {
                internal.rand.choose_copy(&[v1, v3, v2, v4])
            } else {
                choose_weird(&mut internal.rand, [v1, v3, v2, v4])
            };

            temp.insert(temp_index + x_size_rounded, to_insert);

            temp_index += 1;

            v1 = v3;
            v2 = v4;

        }

    }

    for dz in 0..output.z_size {
        let src_offset = (dz + (z & 1) as usize) * x_size_rounded + (x & 1) as usize;
        let dst_offset = dz * output.x_size;
        for dx in 0..output.x_size {
            output.data[dst_offset + dx] = temp[src_offset + dx];
        }
    }

}

fn fuzzy_zoom(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {
    common_zoom(x, z, output, internal, true)
}

fn zoom(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {
    common_zoom(x, z, output, internal, false)
}

impl_layer!(fuzzy_zoom, new_fuzzy_zoom);
impl_layer!(zoom, new_zoom);

impl Layer {
    pub fn new_zoom_multiple(base_seed: i64, mut parent: Self, count: u8) -> Self {
        for i in 0..count {
            parent = Self::new_zoom(base_seed + i as i64, parent);
        }
        parent
    }
}
