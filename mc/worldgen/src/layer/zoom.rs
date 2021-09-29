use super::{Layer, LayerCache, LayerRand};


pub struct ZoomLayer<P: Layer, const FUZZY: bool> {
    pub parent: P,
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
                let v2 = parent.next(x_half + 0, z_half + 1);
                let v3 = parent.next(x_half + 1, z_half + 0);
                let v4 = parent.next(x_half + 1, z_half + 1);
                rand.skip();
                rand.skip();
                if FUZZY {
                    rand.choose(&[v1, v3, v2, v4])
                } else {
                    choose_smart(rand, v1, v3, v2, v4)
                }
            } else if x_odd {
                rand.skip();
                if rand.next_int(2) == 0 {
                    v1
                } else {
                    parent.next(x_half + 1, z_half + 0)
                }
            } else if z_odd {
                if rand.next_int(2) == 0 {
                    v1
                } else {
                    parent.next(x_half + 0, z_half + 1)
                }
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


/// A layer applying a voronoi effect to the biome grid.
pub struct VoronoiLayer<P> {
    pub parent: P,
    rand: LayerRand
}

impl<P> VoronoiLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }
}

impl<P> Layer for VoronoiLayer<P>
where
    P: Layer,
    P::Item: Copy
{

    type Item = P::Item;

    fn seed(&mut self, seed: i64) {
        self.parent.seed(seed);
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        let x = x - 2;
        let z = z - 2;

        let x_new = x >> 2;
        let z_new = z >> 2;

        const VAL: f64 = 4.0 * 0.90000000000000002;

        self.rand.init_chunk_seed((x_new + 0) << 2, (z_new + 0) << 2);
        let a0 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;
        let a1 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;

        self.rand.init_chunk_seed((x_new + 1) << 2, (z_new + 0) << 2);
        let b0 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;
        let b1 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;

        self.rand.init_chunk_seed((x_new + 0) << 2, (z_new + 1) << 2);
        let c0 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL;
        let c1 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;

        self.rand.init_chunk_seed((x_new + 1) << 2, (z_new + 1) << 2);
        let d0 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;
        let d1 = (self.rand.next_int(1024) as f64 / 1024.0 - 0.5) * VAL + 4.0;

        let cdx = (x & 3) as f64;
        let cdz = (z & 3) as f64;

        let a = (cdz - a1) * (cdz - a1) + (cdx - a0) * (cdx - a0);
        let b = (cdz - b1) * (cdz - b1) + (cdx - b0) * (cdx - b0);
        let c = (cdz - c1) * (cdz - c1) + (cdx - c0) * (cdx - c0);
        let d = (cdz - d1) * (cdz - d1) + (cdx - d0) * (cdx - d0);

        if a < b && a < c && a < d {
            self.parent.next(x_new + 0, z_new + 0)
        } else if b < a && b < c && b < d {
            self.parent.next(x_new + 1, z_new + 0)
        } else if c < a && c < b && c < d {
            self.parent.next(x_new + 0, z_new + 1)
        } else {
            self.parent.next(x_new + 1, z_new + 1)
        }

    }

}
