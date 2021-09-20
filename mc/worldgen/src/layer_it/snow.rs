use crate::layer_new::LayerRand;
use super::Layer;

use mc_vanilla::biome::{PLAINS, SNOWY_TUNDRA};
use mc_core::biome::Biome;


pub struct AddSnowLayer<P> {
    parent: P,
    rand: LayerRand
}

impl<P> AddSnowLayer<P> {
    pub fn new(parent: P, base_seed: i64) -> Self {
        Self {
            parent,
            rand: LayerRand::new(base_seed)
        }
    }
}

impl<P> Layer for AddSnowLayer<P>
where
    P: Layer<Item = &'static Biome>
{

    type Item = &'static Biome;

    fn seed(&mut self, seed: i64) {
        self.rand.init_world_seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {
        let mut biome = self.parent.next(x, z);
        if biome == &PLAINS {
            self.rand.init_chunk_seed(x, z);
            if self.rand.next_int(5) == 0 {
                biome = &SNOWY_TUNDRA;
            }
        }
        biome
    }

}
