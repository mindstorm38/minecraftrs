use super::{LayerData, LayerInternal, State};
use crate::biome::def::OCEAN;

fn river_init(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    internal.expect_parent().inner_generate(x, z, output);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {
            internal.rand.init_chunk_seed(x + dx as i32, z + dz as i32);
            let state = output.get_mut(dx, dz);
            *state = if state.expect_biome() == OCEAN::ID {
                State::NoRiver
            } else {
                State::PotentialRiver(internal.rand.next_int(2) as u8 + 2)
            };
        }
    }

}

impl State {
    fn is_no_river(&self) -> bool {
        matches!(*self, State::NoRiver)
    }
}

fn river(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal) {

    let input = internal.expect_parent().generate(x - 1, z - 1, output.x_size + 2, output.z_size + 2);

    for dz in 0..output.z_size {
        for dx in 0..output.x_size {

            let south = input.get(dx + 0, dz + 1);
            let north = input.get(dx + 2, dz + 1);
            let west = input.get(dx + 1, dz + 0);
            let east = input.get(dx + 1, dz + 2);
            let center = input.get(dx + 1, dz + 1);

            if center.is_no_river() || south.is_no_river() || north.is_no_river() || west.is_no_river() || east.is_no_river() {
                output.set(dx, dz, State::River);
            } else if center != south || center != west || center != north || center != east {
                output.set(dx, dz, State::River);
            } else {
                output.set(dx, dz, State::NoRiver);
            }

        }
    }


}

impl_layer!(river_init, new_river_init);
impl_layer!(river, new_river);
