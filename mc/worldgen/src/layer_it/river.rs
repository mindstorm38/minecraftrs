use crate::layer_new::LayerRand;
use super::{Layer, LayerCache};

use mc_vanilla::biome::OCEAN;
use mc_core::biome::Biome;


/// A biome layer that will replace all biomes by 0 if the biome is an `OCEAN`
/// or by a random value in range `[2;4[` that is the probability.
/// This layer is commonly used in parallel with actual biomes generation in is combined
/// with this other layer by a custom layer that take the biome layer and this river layer.
pub struct InitRiverLayer<P> {
    parent: P,
    rand: LayerRand
}

impl<P> InitRiverLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }
}

impl<P> Layer for InitRiverLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = u8;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {
        if self.parent.next(x, z) == &OCEAN {
            0
        } else {
            self.rand.init_chunk_seed(x, z);
            self.rand.next_int(2) as u8 + 2
        }
    }

}

/// A layer that tries to connect rivers.
pub struct AddRiverLayer<P> {
    parent: P,
    cache: LayerCache<bool>
}

impl<P> AddRiverLayer<P> {
    pub fn new(parent: P) -> Self {
        Self {
            parent,
            cache: LayerCache::new()
        }
    }
}

impl<P> Layer for AddRiverLayer<P>
where
    P: Layer<Item = u8>
{

    type Item = bool;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let parent = &mut self.parent;

        *self.cache.get_or_insert(x, z, move || {

            let south = parent.next(x - 1, z);
            let north = parent.next(x + 1, z);
            let west = parent.next(x, z - 1);
            let east = parent.next(x, z + 1);
            let center = parent.next(x, z);

            if center == 0 || south == 0 || north == 0 || west == 0 || east == 0 {
                true
            } else if center != south || center != west || center != north || center != east {
                true
            } else {
                false
            }

        })

    }

}
