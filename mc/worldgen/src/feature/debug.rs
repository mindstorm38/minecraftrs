use mc_core::rand::JavaRandom;
use mc_vanilla::heightmap::WORLD_SURFACE;
use mc_vanilla::block::*;

use super::{Feature, LevelView};


pub struct DebugChunkFeature;

impl Feature for DebugChunkFeature {
    fn generate(&self, chunk: &mut dyn LevelView, _rand: &mut JavaRandom, x: i32, _y: i32, z: i32) {

        let gold = GOLD_BLOCK.get_default_state();
        let diam = DIAMOND_BLOCK.get_default_state();

        for bx in x..(x + 16) {
            chunk.set_block_at(bx, chunk.get_heightmap_column_at(&WORLD_SURFACE, bx, z).unwrap(), z, gold).unwrap();
            chunk.set_block_at(bx, chunk.get_heightmap_column_at(&WORLD_SURFACE, bx, z + 15).unwrap(), z + 15, gold).unwrap();
        }

        for bz in (z + 1)..(z + 15) {
            chunk.set_block_at(x, chunk.get_heightmap_column_at(&WORLD_SURFACE, x, bz).unwrap(), bz, gold).unwrap();
            chunk.set_block_at(x + 15, chunk.get_heightmap_column_at(&WORLD_SURFACE, x + 15, bz).unwrap(), bz, gold).unwrap();
        }

        for bx in (x - 8)..(x + 15 + 8) {
            chunk.set_block_at(bx, chunk.get_heightmap_column_at(&WORLD_SURFACE, bx, z + 4).unwrap(), z + 4, diam).unwrap();
        }

        for bz in (z - 8)..(z + 15 + 8) {
            chunk.set_block_at(x + 4, chunk.get_heightmap_column_at(&WORLD_SURFACE, x + 4, bz).unwrap(), bz, diam).unwrap();
        }

    }
}
