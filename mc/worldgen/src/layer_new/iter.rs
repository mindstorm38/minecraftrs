
use mc_vanilla::biome::{PLAINS, OCEAN};
use mc_core::biome::Biome;

use crate::layer_new::LayerRand;


/// A work-in-progress iterative layer processor, this type of processor only works cell by cell.
pub trait Layer {

    type Item;
    fn seed(&mut self, seed: i64);
    fn next(&mut self, x: i32, z: i32) -> Self::Item;

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Vec<Self::Item> {
        let mut data = Vec::with_capacity(x_size * z_size);
        for z in z..(z + z_size as i32) {
            for x in x..(x + x_size as i32) {
                data.push(self.next(x, z));
            }
        }
        data
    }

}


pub struct IslandLayer {
    rand: LayerRand
}

impl IslandLayer {
    pub fn new(base_seed: i64) -> Self {
        Self {
            rand: LayerRand::new(base_seed)
        }
    }
}

impl Layer for IslandLayer {

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {
        self.rand.init_chunk_seed(x, z);
        match self.rand.next_int(10) {
            0 => &PLAINS,
            _ => &OCEAN
        }
    }

}


pub struct ZoomLayer<P, const FUZZY: bool> {
    parent: P,
    rand: LayerRand
}

impl<P> ZoomLayer<P, true> {

    pub fn new_fuzzy(base_seed: i64, parent: P) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }

}

impl<P> ZoomLayer<P, false> {

    pub fn new_smart(base_seed: i64, parent: P) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }

}

impl<P, const FUZZY: bool> Layer for ZoomLayer<P, FUZZY>
where
    P: Layer,
    P::Item: Copy + PartialEq
{

    type Item = P::Item;

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        // Note that the conversion from the old algorithm that worked in a buffer is WIP.

        let x_half = x >> 1;
        let z_half = z >> 1;

        let x_odd = (x & 1) == 1;
        let z_odd = (z & 1) == 1;

        let v1 = self.parent.next(x_half, z_half);

        self.rand.init_chunk_seed(x, z);

        if x_odd && z_odd {
            let v2 = self.parent.next(x_half, z_half + 1);
            let v3 = self.parent.next(x_half + 1, z_half);
            let v4 = self.parent.next(x_half + 1, z_half + 1);
            self.rand.skip();
            self.rand.skip();
            if FUZZY {
                self.rand.choose(&[v1, v3, v2, v4])
            } else {
                choose_smart(&mut self.rand, v1, v3, v2, v4)
            }
        } else if x_odd {
            let v3 = self.parent.next(x_half + 1, z_half);
            self.rand.skip();
            self.rand.choose(&[v1, v3])
        } else if z_odd {
            let v2 = self.parent.next(x_half, z_half + 1);
            self.rand.choose(&[v1, v2])
        } else {
            v1
        }

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
