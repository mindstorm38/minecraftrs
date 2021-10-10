use mc_core::block::BlockState;
use mc_core::heightmaps;

use crate::block::AIR;


fn heightmap_world_surface(state: &'static BlockState) -> bool {
    state != AIR.get_default_state()
}

fn heightmap_ocean_floor(state: &'static BlockState) -> bool {
    // TODO: In reality this returns true if the block is motion blocking.
    state != AIR.get_default_state()
}

fn heightmap_motion_blocking(state: &'static BlockState) -> bool {
    // TODO: Complete the '||' and is motion blocking
    state != AIR.get_default_state() // || state is fluid
}

fn heightmap_motion_blocking_no_leaves(state: &'static BlockState) -> bool {
    heightmap_motion_blocking(state) // && state is not leaves
}

heightmaps!(pub VANILLA_HEIGHTMAPS [
    WORLD_SURFACE "WORLD_SURFACE" heightmap_world_surface,
    OCEAN_FLOOR "OCEAN_FLOOR" heightmap_ocean_floor,
    MOTION_BLOCKING "MOTION_BLOCKING" heightmap_motion_blocking,
    MOTION_BLOCKING_NO_LEAVES "MOTION_BLOCKING_NO_LEAVES" heightmap_motion_blocking_no_leaves
]);
