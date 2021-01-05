use super::{LayerData, LayerInternal, State};

fn voronoi(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let x = x - 2;
    let z = z - 2;

    let x_new = x >> 2;
    let z_new = z >> 2;
    let x_size_new = (output.x_size >> 2) + 3;
    let z_size_new = (output.z_size >> 2) + 3;
    let x_size_rounded = x_size_new << 2;
    let z_size_rounded = z_size_new << 2;

    let input = internal.expect_parent().generate(x_new, z_new, x_size_new, z_size_new);

    // Try to replace this with LayerData
    let mut temp: Vec<State> = vec![State::Uninit; x_size_rounded * z_size_rounded];

    for dz in 0..(z_size_new - 1) {

        let mut v1 = input.get(0, dz + 0);
        let mut v2 = input.get(0, dz + 1);

        for dx in 0..(x_size_new - 1) {

            const VAL: f64 = 4.0 * 0.90000000000000002;

            internal.rand.init_chunk_seed((x_new + dx as i32) << 2, (z_new + dz as i32) << 2);
            let a0 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;
            let a1 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;

            internal.rand.init_chunk_seed((x_new + dx as i32 + 1) << 2, (z_new + dz as i32) << 2);
            let b0 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;
            let b1 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;

            internal.rand.init_chunk_seed((x_new + dx as i32) << 2, (z_new + dz as i32 + 1) << 2);
            let c0 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;
            let c1 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;

            internal.rand.init_chunk_seed((x_new + dx as i32 + 1) << 2, (z_new + dz as i32 + 1) << 2);
            let d0 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;
            let d1 = (internal.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;

            let v3 = input.get(dx + 1, dz + 0);
            let v4 = input.get(dx + 1, dz + 1);

            for cdz in 0..4 {
                let mut temp_index = ((dz << 2) + cdz) * x_size_rounded + (dx << 2);
                for cdx in 0..4 {

                    let cdz = cdz as f64;
                    let cdx = cdx as f64;

                    let a = (cdz - a1) * (cdz  - a1) + (cdx - a0) * (cdx - a0);
                    let b = (cdz - b1) * (cdz  - b1) + (cdx - b0) * (cdx - b0);
                    let c = (cdz - c1) * (cdz  - c1) + (cdx - c0) * (cdx - c0);
                    let d = (cdz - d1) * (cdz  - d1) + (cdx - d0) * (cdx - d0);

                    let to_set = if a < b && a < c && a < d {
                        v1
                    } else if b < a && b < c && b < d {
                        v3
                    } else if c < a && c < b && c < d {
                        v2
                    } else {
                        v4
                    };

                    temp[temp_index] = to_set;
                    temp_index += 1;

                }
            }

            v1 = v3;
            v2 = v4;

        }

    }

    for dz in 0..output.z_size {
        let src_offset = (dz + (z & 7) as usize) * x_size_rounded + (x & 7) as usize;
        let dst_offset = dz * output.x_size;
        for dx in 0..output.x_size {
            output.data[dst_offset + dx] = temp[src_offset + dx];
        }
    }

    output.debug("voronoi_zoom");

}

impl_layer!(voronoi, new_voronoi);
