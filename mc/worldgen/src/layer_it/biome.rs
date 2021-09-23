use crate::layer_new::LayerRand;
use super::Layer;

use mc_core::biome::Biome;
use mc_vanilla::biome::{
    PLAINS, DESERT, FOREST, MOUNTAINS, SWAMP, TAIGA, JUNGLE, DESERT_HILLS, WOODED_HILLS,
    TAIGA_HILLS, SNOWY_TUNDRA, SNOWY_MOUNTAINS, JUNGLE_HILLS, MUSHROOM_FIELDS, OCEAN,
    MUSHROOM_FIELD_SHORE, RIVER, BEACH, MOUNTAIN_EDGE, FROZEN_RIVER
};


/// This layer replace all incoming 'plains' biome by a random biome chosen
/// from the internal biomes slice give when constructing the layer.
pub struct BiomeLayer<P> {
    parent: P,
    rand: LayerRand,
    biomes: &'static [&'static Biome]
}

impl<P> BiomeLayer<P> {

    pub fn new(parent: P, base_seed: i64, biomes: &'static [&'static Biome]) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed),
            biomes
        }
    }

    pub fn from_version(parent: P, base_seed: i64, version: (u8, u8)) -> Option<Self> {

        static BIOMES_1_2: [&'static Biome; 7] = [
            &DESERT,
            &FOREST,
            &MOUNTAINS,  // Extreme hills before 1.13
            &SWAMP,
            &PLAINS,
            &TAIGA,
            &JUNGLE
        ];

        match version {
            (1, 2) => Some(Self::new(parent, base_seed, &BIOMES_1_2)),
            _ => None
        }

    }

}

impl<P> Layer for BiomeLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {
        let biome = self.parent.next(x, z);
        if biome == &PLAINS {
            self.rand.init_chunk_seed(x, z);
            self.rand.choose(self.biomes)
        } else {
            biome
        }
    }

}
