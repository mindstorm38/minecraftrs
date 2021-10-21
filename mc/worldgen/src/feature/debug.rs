use mc_core::rand::JavaRandom;
use mc_vanilla::heightmap::WORLD_SURFACE;
use mc_vanilla::block::*;

use super::{Feature, LevelView};


pub struct DebugChunkFeature;

impl Feature for DebugChunkFeature {
    fn generate(&self, chunk: &mut dyn LevelView, _rand: &mut JavaRandom, x: i32, _y: i32, z: i32) {

        for bx in x..(x + 16) {
            chunk.set_block_at(bx, chunk.get_heightmap_column_at(&WORLD_SURFACE, bx, z).unwrap(), z, GOLD_BLOCK.get_default_state());
            chunk.set_block_at(bx, chunk.get_heightmap_column_at(&WORLD_SURFACE, bx, z + 15).unwrap(), z + 15, GOLD_BLOCK.get_default_state());
        }

        for bz in (z + 1)..(z + 15) {
            chunk.set_block_at(x, chunk.get_heightmap_column_at(&WORLD_SURFACE, x, bz).unwrap(), bz, GOLD_BLOCK.get_default_state());
            chunk.set_block_at(x + 15, chunk.get_heightmap_column_at(&WORLD_SURFACE, x + 15, bz).unwrap(), bz, GOLD_BLOCK.get_default_state());
        }

    }
}
