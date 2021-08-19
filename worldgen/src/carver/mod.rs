use crate::rand::jrand::JavaRandom;
use crate::world::chunk::Chunk;
use std::num::Wrapping;


pub struct CarverInternal {
    pub rand: JavaRandom,
    pub range: i32
}

pub type CarverHandleFn = fn(ccx: i32, ccz: i32, chunk: &mut Chunk, internal: &mut CarverInternal);

pub struct Carver {
    internal: CarverInternal,
    handler: CarverHandleFn
}

impl Carver {

    pub fn new(handler: CarverHandleFn, range: i32) -> Self {
        Carver {
            internal: CarverInternal {
                rand: JavaRandom::new_blank(),
                range
            },
            handler
        }
    }

    pub fn generate(&mut self, chunk: &mut Chunk) {

        let word_seed = chunk.get_world_info().seed;

        self.internal.rand.set_seed(word_seed);

        let x_rand = Wrapping(self.internal.rand.next_long());
        let z_rand = Wrapping(self.internal.rand.next_long());

        let (cx, cz) = chunk.get_position();
        let range = self.internal.range;

        for ccx in (cx - range)..=(cx + range) {
            for ccz in (cz - range)..=(cz + range) {

                let seed = (Wrapping(ccx as i64) * x_rand) ^ (Wrapping(ccz as i64) * z_rand) ^ Wrapping(word_seed);
                self.internal.rand.set_seed(seed.0);

                (self.handler)(ccx, ccz, chunk, &mut self.internal);

            }
        }

    }

}

macro_rules! impl_carver {
    ($func:ident, $new_func:ident, $range:expr) => {
        impl $crate::world::gen::carver::Carver {
            #[inline]
            pub fn $new_func() -> Self {
                Self::new($func, $range)
            }
        }
    };
}

pub mod ravine;
pub mod cave;
