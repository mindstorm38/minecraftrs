use std::collections::HashMap;

use mc_core::biome::{BiomeKey, Biome};
use mc_core::block::{BlockState, Block};
use mc_vanilla::block::*;

use crate::feature::{FeatureChain, Feature};
use crate::feature::vein::GenVeinFeature;
// use crate::feature::debug::DebugChunkFeature;


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

pub struct BiomePropertyBuilder {
    biome: &'static Biome,
    inner: BiomeProperty,
    map: BiomePropertyMap,
}

impl BiomePropertyBuilder {

    pub fn height(mut self, min: f32, max: f32) -> Self {
        self.inner.min_height = min;
        self.inner.max_height = max;
        self
    }

    pub fn temp(mut self, temp: f32) -> Self {
        self.inner.temperature = temp;
        self
    }

    pub fn blocks(mut self, top: &'static Block, filler: &'static Block) -> Self {
        self.inner.top_block = top.get_default_state();
        self.inner.filler_block = filler.get_default_state();
        self
    }

    pub fn build(mut self) -> BiomePropertyMap {
        self.map.data.insert(self.biome.get_key(), self.inner);
        self.map
    }

}

impl BiomePropertyMap {

    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn insert(self, biome: &'static Biome) -> BiomePropertyBuilder {
        BiomePropertyBuilder {
            biome,
            inner: BiomeProperty {
                min_height: 0.1,
                max_height: 0.3,
                temperature: 0.5,
                top_block: GRASS_BLOCK.get_default_state(),
                filler_block: DIRT.get_default_state(),
                features: get_common_features()
            },
            map: self
        }
    }

    pub fn get(&self, biome: &'static Biome) -> Option<&BiomeProperty> {
        self.data.get(&biome.get_key())
    }

    pub fn get_height(&self, biome: &'static Biome) -> Option<(f32, f32)> {
        self.get(biome).map(|prop| (prop.min_height, prop.max_height))
    }

}


fn get_common_features() -> FeatureChain {
    let mut chain = FeatureChain::new();
    chain.push(GenVeinFeature::new(DIRT.get_default_state(), 32).distributed_uniform(0, 128).repeated(20));
    chain.push(GenVeinFeature::new(GRAVEL.get_default_state(), 32).distributed_uniform(0, 128).repeated(10));
    chain.push(GenVeinFeature::new(COAL_ORE.get_default_state(), 16).distributed_uniform(0, 128).repeated(20));
    chain.push(GenVeinFeature::new(IRON_ORE.get_default_state(), 8).distributed_uniform(0, 64).repeated(20));
    chain.push(GenVeinFeature::new(GOLD_ORE.get_default_state(), 8).distributed_uniform(0, 32).repeated(2));
    chain.push(GenVeinFeature::new(REDSTONE_ORE.get_default_state(), 7).distributed_uniform(0, 16).repeated(8));
    chain.push(GenVeinFeature::new(DIAMOND_ORE.get_default_state(), 7).distributed_uniform(0, 16));
    chain.push(GenVeinFeature::new(LAPIS_ORE.get_default_state(), 6).distributed_triangular(16, 16));
    // chain.push(DebugChunkFeature);
    chain
}
