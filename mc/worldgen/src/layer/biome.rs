use super::{Layer, LayerCache, LayerRand};

use mc_core::biome::Biome;
use mc_vanilla::biome::{
    PLAINS, DESERT, FOREST, MOUNTAINS, SWAMP, TAIGA, JUNGLE, DESERT_HILLS, WOODED_HILLS,
    TAIGA_HILLS, SNOWY_TUNDRA, SNOWY_MOUNTAINS, JUNGLE_HILLS, MUSHROOM_FIELDS, OCEAN,
    MUSHROOM_FIELD_SHORE, RIVER, BEACH, MOUNTAIN_EDGE, FROZEN_RIVER
};


/// This layer replace all incoming 'plains' biome by a random biome chosen
/// from the internal biomes slice give when constructing the layer.
pub struct BiomeLayer<P> {
    pub parent: P,
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

    pub fn with_version(parent: P, base_seed: i64, version: (u8, u8)) -> Option<Self> {
        match version {
            (1, 2) => Some(Self::with_version_1_2(parent, base_seed)),
            _ => None
        }
    }

    pub fn with_version_1_2(parent: P, base_seed: i64) -> Self {
        static BIOMES: [&'static Biome; 7] = [
            &DESERT,
            &FOREST,
            &MOUNTAINS,  // Extreme hills before 1.13
            &SWAMP,
            &PLAINS,
            &TAIGA,
            &JUNGLE
        ];
        Self::new(parent, base_seed, &BIOMES)
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


/// This layer convert 1/3 of the non-hills biomes to their hills variant if the
/// hill is in between normal variant on the 4 sides.
pub struct HillsLayer<P> {
    pub parent: P,
    rand: LayerRand,
    repl_cache: LayerCache<&'static Biome>
}

impl<P> HillsLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed),
            repl_cache: LayerCache::new()
        }
    }
}

impl<P> Layer for HillsLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.rand.init_world_seed(seed);
        self.repl_cache.clear();
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let mut biome = self.parent.next(x, z);

        self.rand.init_chunk_seed(x, z);
        if self.rand.next_int(3) == 0 {

            let repl = match biome {
                _ if biome == &DESERT => Some(&DESERT_HILLS),
                _ if biome == &FOREST => Some(&WOODED_HILLS),  // Forest hills before 1.13
                _ if biome == &TAIGA => Some(&TAIGA_HILLS),
                _ if biome == &PLAINS => Some(&FOREST),
                _ if biome == &SNOWY_TUNDRA => Some(&SNOWY_MOUNTAINS),
                _ if biome == &JUNGLE => Some(&JUNGLE_HILLS),
                _ => None
            };

            if let Some(repl) = repl {

                let parent = &mut self.parent;

                biome = *self.repl_cache.get_or_insert(x, z, move || {

                    let south = parent.next(x - 1, z);
                    let north = parent.next(x + 1, z);
                    let west = parent.next(x, z - 1);
                    let east = parent.next(x, z + 1);

                    if south == biome && north == biome && west == biome && east == biome {
                        repl
                    } else {
                        biome
                    }

                })

            }

        }

        biome

    }

}


/// A layer that add some shore and edge biomes depending on island and hills placements.
pub struct ShoreLayer<P> {
    pub parent: P,
    cache: LayerCache<&'static Biome>
}

impl<P> ShoreLayer<P> {
    pub fn new(parent: P) -> Self {
        Self {
            parent,
            cache: LayerCache::new()
        }
    }
}


impl<P> Layer for ShoreLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.cache.clear();
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let parent = &mut self.parent;

        *self.cache.get_or_insert(x, z, move || {

            macro_rules! south {() => { parent.next(x - 1, z) }}
            macro_rules! north {() => { parent.next(x + 1, z) }}
            macro_rules! west {() => { parent.next(x, z - 1) }}
            macro_rules! east {() => { parent.next(x, z + 1) }}

            let mut center = parent.next(x, z);

            if center == &MUSHROOM_FIELDS {
                if south!() == &OCEAN && north!() == &OCEAN && west!() == &OCEAN && east!() == &OCEAN {
                    center = &MUSHROOM_FIELD_SHORE;
                }
            } else if center != &OCEAN && center != &RIVER && center != &SWAMP && center != &MOUNTAINS {
                if south!() == &OCEAN || north!() == &OCEAN || west!() == &OCEAN || east!() == &OCEAN {
                    center = &BEACH;
                }
            } else if center == &MOUNTAINS {
                if south!() != &MOUNTAINS || north!() != &MOUNTAINS || west!() != &MOUNTAINS || east!() != &MOUNTAINS {
                    center = &MOUNTAIN_EDGE;
                }
            }

            center

        })

    }

}


/// A layer that adds rivers in swamps and jungles.
pub struct BiomeRiverLayer<P> {
    pub parent: P,
    rand: LayerRand
}

impl<P> BiomeRiverLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }
}

impl<P> Layer for BiomeRiverLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let mut biome = self.parent.next(x, z);

        self.rand.init_chunk_seed(x, z);

        if biome == &SWAMP && self.rand.next_int(6) == 0 {
            biome = &RIVER;
        } else if (biome == &JUNGLE || biome == &JUNGLE_HILLS) && self.rand.next_int(8) == 0 {
            biome = &RIVER;
        }

        biome

    }

}


/// A layer that mix two parent layers, the parent #0 must be the biome layer and
/// the #1 the river layer.
pub struct MixBiomeAndRiverLayer<B, R> {
    pub biome_parent: B,
    pub river_parent: R
}

impl<B, R> MixBiomeAndRiverLayer<B, R> {
    pub fn new(biome_parent: B, river_parent: R) -> Self {
        Self {
            biome_parent,
            river_parent
        }
    }
}

impl<B, R> Layer for MixBiomeAndRiverLayer<B, R>
where
    B: Layer<Item = &'static Biome>,
    R: Layer<Item = bool>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.biome_parent.seed(seed);
        self.river_parent.seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let mut biome = self.biome_parent.next(x, z);

        if biome != &OCEAN && self.river_parent.next(x, z) {
            biome = match () {
                _ if biome == &SNOWY_TUNDRA => &FROZEN_RIVER,
                _ if biome == &MUSHROOM_FIELDS => &MUSHROOM_FIELD_SHORE,
                _ if biome == &MUSHROOM_FIELD_SHORE => &MUSHROOM_FIELD_SHORE,
                _ => &RIVER
            };
        }

        biome

    }

}
