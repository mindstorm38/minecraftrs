use super::{Layer, LayerCache, LayerRand};

/// This layer smooth the input layer by removed micro biomes and filling
/// gaps between cells of the same biome.
pub struct SmoothLayer<P: Layer> {
    pub parent: P,
    rand: LayerRand,
    cache: LayerCache<P::Item>
}

impl<P: Layer> SmoothLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed),
            cache: LayerCache::new()
        }
    }
}

impl<P: Layer> Layer for SmoothLayer<P>
where
    P::Item: Copy + Eq
{

    type Item = P::Item;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.rand.init_world_seed(seed);
        self.cache.clear();
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let parent = &mut self.parent;
        let rand = &mut self.rand;

        *self.cache.get_or_insert(x, z, move || {

            let south = parent.next(x - 1, z);
            let north = parent.next(x + 1, z);
            let west = parent.next(x, z - 1);
            let east = parent.next(x, z + 1);
            let center = parent.next(x, z);

            if south == north && west == east {
                rand.init_chunk_seed(x, z);
                if rand.next_int(2) == 0 {
                    south
                } else {
                    west
                }
            } else if west == east {
                west
            } else if south == north {
                south
            } else {
                center
            }

        })

    }

}
