use crate::block::vanilla::VanillaBlocks;
use crate::block::BlockState;
use crate::heightmaps;


fn predicate_not_air(state: &'static BlockState) -> bool {
    state == VanillaBlocks.AIR.get_default_state()
}


heightmaps! {
    pub WORLD_SURFACE_WG: predicate_not_air;
    pub WORLD_SURFACE: predicate_not_air;
    pub OCEAN_FLOOR_WG: predicate_not_air;  // TODO
    pub OCEAN_FLOOR: predicate_not_air;     // TODO
    pub MOTION_BLOCKING: predicate_not_air; // TODO
    pub MOTION_BLOCKING_NO_LEAVES: predicate_not_air; // TODO
}
