use mc_vanilla::heightmap::WORLD_SURFACE;
use mc_vanilla::block::*;
use mc_core::block::BlockState;
use mc_core::rand::JavaRandom;

use crate::view::LevelView;
use super::Feature;


pub struct DebugChunkFeature;

impl Feature for DebugChunkFeature {
    fn generate(&self, level: &mut dyn LevelView, _rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> bool {

        let gold = GOLD_BLOCK.get_default_state();

        for bx in x..(x + 16) {
            level.set_block_at(bx, level.get_heightmap_column_at(&WORLD_SURFACE, bx, z).unwrap(), z, gold).unwrap();
            level.set_block_at(bx, level.get_heightmap_column_at(&WORLD_SURFACE, bx, z + 15).unwrap(), z + 15, gold).unwrap();
        }

        for bz in (z + 1)..(z + 15) {
            level.set_block_at(x, level.get_heightmap_column_at(&WORLD_SURFACE, x, bz).unwrap(), bz, gold).unwrap();
            level.set_block_at(x + 15, level.get_heightmap_column_at(&WORLD_SURFACE, x + 15, bz).unwrap(), bz, gold).unwrap();
        }

        true

    }
}


pub struct SetBlockFeature {
    block: &'static BlockState
}

impl SetBlockFeature {
    pub fn new(block: &'static BlockState) -> Self {
        Self { block }
    }
}

impl Feature for SetBlockFeature {
    fn generate(&self, level: &mut dyn LevelView, _rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        level.set_block_at(x, y, z, self.block).unwrap();
        true
    }
}