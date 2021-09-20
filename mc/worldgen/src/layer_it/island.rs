use crate::layer_new::LayerRand;
use super::Layer;

use mc_vanilla::biome::{PLAINS, OCEAN, SNOWY_TUNDRA, FROZEN_OCEAN};
use mc_core::biome::Biome;


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
        // TODO: Check to implements forced plains biomes at 0/0
    }

}


pub struct AddIslandLayer<P> {
    parent: P,
    rand: LayerRand,
}

impl<P> AddIslandLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }
}

impl<P> Layer for AddIslandLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {

        macro_rules! post_inc {
            ($v:ident) => (($v, $v += 1).0);
        }

        let center = self.parent.next(x, z);
        let sw = self.parent.next(x - 1, z - 1);
        let nw = self.parent.next(x + 1, z - 1);
        let se = self.parent.next(x - 1, z + 1);
        let ne = self.parent.next(x + 1, z + 1);

        self.rand.init_chunk_seed(x, z);

        if is_ocean(center) && (!is_ocean(sw) || !is_ocean(nw) || !is_ocean(se) || !is_ocean(ne)) {

            let mut bound = 1;
            let mut to_set = &PLAINS;

            if !is_ocean(sw) && self.rand.next_int(post_inc!(bound)) == 0 {
                to_set = sw;
            }

            if !is_ocean(nw) && self.rand.next_int(post_inc!(bound)) == 0 {
                to_set = nw;
            }

            if !is_ocean(se) && self.rand.next_int(post_inc!(bound)) == 0 {
                to_set = se;
            }

            if !is_ocean(ne) && self.rand.next_int(bound) == 0 {
                to_set = ne;
            }

            if self.rand.next_int(3) == 0 {
                to_set
            } else if to_set == &SNOWY_TUNDRA {
                // Snowy Tundra is the modern name of Ice plains
                &FROZEN_OCEAN
            } else {
                &OCEAN
            }

        } else if !is_ocean(center) && (is_ocean(sw) || is_ocean(nw) || is_ocean(se) || is_ocean(ne)) {
            if self.rand.next_int(5) == 0 {
                if center == &SNOWY_TUNDRA {
                    &FROZEN_OCEAN
                } else {
                    &OCEAN
                }
            } else {
                center
            }
        } else {
            center
        }

    }

}


#[inline]
fn is_ocean(biome: &'static Biome) -> bool {
    biome == &OCEAN
}
