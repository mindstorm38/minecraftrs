use std::collections::HashMap;

use mc_core::biome::{BiomeKey, Biome};
use mc_core::block::{BlockState, Block};

use crate::feature::{FeatureChain};


pub struct BiomePropertyMap {
    data: HashMap<BiomeKey, BiomeProperty>
}

pub struct BiomeProperty {
    pub min_height: f32,
    pub max_height: f32,
    pub temperature: f32,
    pub top_block: &'static BlockState,
    pub filler_block: &'static BlockState,
    pub features: FeatureChain
}

impl BiomePropertyMap {

    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, biome: &'static Biome, prop: BiomeProperty) {
        self.data.insert(biome.get_key(), prop);
    }

    pub fn get(&self, biome: &'static Biome) -> Option<&BiomeProperty> {
        self.data.get(&biome.get_key())
    }

    pub fn get_height(&self, biome: &'static Biome) -> Option<(f32, f32)> {
        self.get(biome).map(|prop| (prop.min_height, prop.max_height))
    }

}

impl BiomeProperty {

    pub fn height(mut self, min: f32, max: f32) -> Self {
        self.min_height = min;
        self.max_height = max;
        self
    }

    pub fn temp(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    pub fn blocks(mut self, top: &'static Block, filler: &'static Block) -> Self {
        self.top_block = top.get_default_state();
        self.filler_block = filler.get_default_state();
        self
    }

}
