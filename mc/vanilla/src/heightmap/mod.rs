use mc_core::block::{BlockState, GlobalBlocks};
use mc_core::heightmaps;

use crate::block::material::{TAG_LIQUID, TAG_NON_BLOCKING, TAG_LEAVES};
use crate::block::AIR;


fn heightmap_world_surface(state: &'static BlockState, _blocks: &GlobalBlocks) -> bool {
    state != AIR.get_default_state()
}

fn heightmap_ocean_floor(state: &'static BlockState, blocks: &GlobalBlocks) -> bool {
    !blocks.has_block_tag(state.get_block(), &TAG_NON_BLOCKING)
}

fn heightmap_ocean_floor_wg(state: &'static BlockState, blocks: &GlobalBlocks) -> bool {
    heightmap_ocean_floor(state, blocks) && !blocks.has_block_tag(state.get_block(), &TAG_LEAVES)
}

fn heightmap_motion_blocking(state: &'static BlockState, blocks: &GlobalBlocks) -> bool {
    let block = state.get_block();
    !blocks.has_block_tag(block, &TAG_NON_BLOCKING) || blocks.has_block_tag(block, &TAG_LIQUID)
}

fn heightmap_motion_blocking_no_leaves(state: &'static BlockState, blocks: &GlobalBlocks) -> bool {
    heightmap_motion_blocking(state, blocks) && !blocks.has_block_tag(state.get_block(), &TAG_LEAVES)
}

heightmaps!(pub VANILLA_HEIGHTMAPS [
    WORLD_SURFACE "WORLD_SURFACE" heightmap_world_surface,
    OCEAN_FLOOR "OCEAN_FLOOR" heightmap_ocean_floor,
    OCEAN_FLOOR_WG "OCEAN_FLOOR_WG" heightmap_ocean_floor_wg,
    MOTION_BLOCKING "MOTION_BLOCKING" heightmap_motion_blocking,
    MOTION_BLOCKING_NO_LEAVES "MOTION_BLOCKING_NO_LEAVES" heightmap_motion_blocking_no_leaves
]);
