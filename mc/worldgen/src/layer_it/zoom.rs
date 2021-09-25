use crate::layer_new::LayerRand;
use super::{Layer, LayerCache};


pub struct ZoomLayer<P: Layer, const FUZZY: bool> {
    parent: P,
    rand: LayerRand,
    cache: LayerCache<P::Item>
}

impl<P: Layer> ZoomLayer<P, true> {

    pub fn new_fuzzy(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed),
            cache: LayerCache::new()
        }
    }

}

impl<P: Layer> ZoomLayer<P, false> {

    pub fn new_smart(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed),
            cache: LayerCache::new()
        }
    }

}

impl<P: Layer, const FUZZY: bool> Layer for ZoomLayer<P, FUZZY>
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

            let x_half = x >> 1;
            let z_half = z >> 1;

            let x_odd = (x & 1) == 1;
            let z_odd = (z & 1) == 1;

            let v1 = parent.next(x_half, z_half);

            rand.init_chunk_seed(x_half << 1, z_half << 1);

            if x_odd && z_odd {
                let v2 = parent.next(x_half, z_half + 1);
                let v3 = parent.next(x_half + 1, z_half);
                let v4 = parent.next(x_half + 1, z_half + 1);
                rand.skip();
                rand.skip();
                if FUZZY {
                    rand.choose(&[v1, v3, v2, v4])
                } else {
                    choose_smart(rand, v1, v3, v2, v4)
                }
            } else if x_odd {
                let v3 = parent.next(x_half + 1, z_half);
                rand.skip();
                rand.choose(&[v1, v3])
            } else if z_odd {
                let v2 = parent.next(x_half, z_half + 1);
                rand.choose(&[v1, v2])
            } else {
                v1
            }

        })

    }

}

/// Internal method to choose from 4 states the most commonly represented.
#[inline]
fn choose_smart<T>(rand: &mut LayerRand, v1: T, v2: T, v3: T, v4: T) -> T
    where
        T: Copy + PartialEq
{

    if v2 == v3 && v3 == v4 {
        v2
    } else if v1 == v2 && v1 == v3 {
        v1
    } else if v1 == v2 && v1 == v4 {
        v1
    } else if v1 == v3 && v1 == v4 {
        v1
    } else if v1 == v2 && v3 != v4 {
        v1
    } else if v1 == v3 && v2 != v4 {
        v1
    } else if v1 == v4 && v2 != v3 {
        v1
    } else if v2 == v1 && v3 != v4 {
        v2
    } else if v2 == v3 && v1 != v4 {
        v2
    } else if v2 == v4 && v1 != v3 {
        v2
    } else if v3 == v1 && v2 != v4 {
        v3
    } else if v3 == v2 && v1 != v4 {
        v3
    } else if v3 == v4 && v1 != v2 {
        v3
    } else if v4 == v1 && v2 != v3 {
        v3 // As in MCP 1.2.5, but weird
    } else if v4 == v2 && v1 != v3 {
        v3 // As in MCP 1.2.5, but weird
    } else if v4 == v3 && v1 != v2 {
        v3 // As in MCP 1.2.5, but weird
    } else {
        rand.choose(&[v1, v2, v3, v4])
    }

}
