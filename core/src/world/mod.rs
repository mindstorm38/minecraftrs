use std::marker::PhantomPinned;
use std::pin::Pin;

use crate::block::WorkBlocks;
use crate::biome::WorkBiomes;


pub struct World {
    blocks: WorkBlocks<'static>,
    biomes: WorkBiomes<'static>,
    marker: PhantomPinned
}

impl World {

    pub fn new(blocks: WorkBlocks<'static>, biomes: WorkBiomes<'static>) -> Pin<Box<World>> {

        let world = Box::pin(World {
            blocks,
            biomes,
            marker: PhantomPinned
        });

        world

    }

    pub fn new_vanilla() -> Pin<Box<World>> {
        Self::new(WorkBlocks::new_vanilla(), WorkBiomes::new_vanilla())
    }

    pub fn get_blocks(&self) -> &WorkBlocks<'static> {
        &self.blocks
    }

    pub fn get_biomes(&self) -> &WorkBiomes<'static> {
        &self.biomes
    }

}
