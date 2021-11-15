use mc_core::block::{Block, BlockState};
use mc_core::rand::JavaRandom;

use mc_vanilla::block::*;
use mc_vanilla::block::material::TAG_LEAVES;

use crate::view::LevelView;

use super::Feature;


pub struct PlantFeature {
    block: &'static BlockState,
    try_count: u32,
    // search_floor: bool,
    can_plant_predicate: fn(&'static Block) -> bool
}

impl PlantFeature {

    pub fn new(block: &'static Block, try_count: u32, /*search_floor: bool,*/ can_plant_predicate: fn(&'static Block) -> bool) -> Self {
        Self {
            block: block.get_default_state(),
            try_count,
            // search_floor,
            can_plant_predicate
        }
    }

    pub fn new_flower(block: &'static Block) -> Self {
        Self::new(block, 64, can_plant_living)
    }

    pub fn new_grass(block: &'static Block) -> Self {
        Self::new(block, 128, can_plant_living)
    }

    pub fn new_dead_bush() -> Self {
        Self::new(&DEAD_BUSH, 4, can_plant_dead)
    }

    pub fn new_lily_pad() -> Self {
        Self::new(&LILY_PAD, 10, can_plant_lily_pad)
    }

}

impl Feature for PlantFeature {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, mut y: i32, z: i32) -> bool {
        for _ in 0..self.try_count {

            let bx = (x + rand.next_int_bounded(8)) - rand.next_int_bounded(8);
            let by = (y + rand.next_int_bounded(4)) - rand.next_int_bounded(4);
            let bz = (z + rand.next_int_bounded(8)) - rand.next_int_bounded(8);

            if by > 0 && by < 256 {
                let current_block = level.get_block_at(bx, by, bz).unwrap().get_block();
                if current_block == &AIR /* && (block light >= 8 || see sky) */ {
                    let ground_block = level.get_block_at(bx, by - 1, bz).unwrap().get_block();
                    if (self.can_plant_predicate)(ground_block) {
                        level.set_block_at(bx, by, bz, self.block).unwrap();
                    }
                }
            }

        }
        true
    }

}

fn can_plant_living(block: &'static Block) -> bool {
    block == &GRASS_BLOCK || block == &DIRT || block == &FARMLAND
}

fn can_plant_dead(block: &'static Block) -> bool {
    block == &SAND
}

fn can_plant_lily_pad(block: &'static Block) -> bool {
    block == &WATER
}


pub struct SugarCaneFeature;

impl Feature for SugarCaneFeature {

    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        for _ in 0..20 {

            let bx = (x + rand.next_int_bounded(4)) - rand.next_int_bounded(4);
            let bz = (z + rand.next_int_bounded(4)) - rand.next_int_bounded(4);

            if level.get_block_at(bx, y, bz).unwrap().is_block(&AIR) && (
                level.get_block_at(bx - 1, y - 1, bz + 0).unwrap().is_block(&WATER) ||
                level.get_block_at(bx + 1, y - 1, bz + 0).unwrap().is_block(&WATER) ||
                level.get_block_at(bx + 0, y - 1, bz - 1).unwrap().is_block(&WATER) ||
                level.get_block_at(bx + 0, y - 1, bz + 1).unwrap().is_block(&WATER)
            ) {
                let height = rand.next_int_bounded(3);
                let height = rand.next_int_bounded(height + 1) + 2;
                let ground_block = level.get_block_at(bx, y - 1, bz).unwrap().get_block();
                if ground_block == &GRASS_BLOCK || ground_block == &DIRT || ground_block == &SAND {
                    for by in y..(y + height) {
                        level.set_block_at(bx, by, bz, SUGAR_CANE.get_default_state()).unwrap();
                    }
                }
            }

        }
        true
    }

}
