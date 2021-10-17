use std::num::Wrapping;

use mc_core::rand::JavaRandom;
use mc_core::world::chunk::Chunk;

pub mod cave;
pub mod ravine;


/// Base trait for all structure implementations, including carvers, village or strongholds.
pub trait Structure {
    fn generate(&mut self, ccx: i32, ccz: i32, chunk: &mut Chunk, range: i32, rand: &mut JavaRandom);
}


/// A functional wrapper for `Structure` with a local random and range.
pub struct StructureGenerator<S: Structure> {
    range: i32,
    structure: S
}

impl<S: Structure> StructureGenerator<S> {

    pub fn new(structure: S, range: i32) -> Self {
        Self {
            range,
            structure
        }
    }

    pub fn generate(&mut self, seed: i64, chunk: &mut Chunk) {

        let mut rand = JavaRandom::new(seed);

        let x_rand = Wrapping(rand.next_long());
        let z_rand = Wrapping(rand.next_long());

        let (cx, cz) = chunk.get_position();

        for ccx in (cx - self.range)..=(cx + self.range) {
            for ccz in (cz - self.range)..=(cz + self.range) {

                let seed = (Wrapping(ccx as i64) * x_rand) ^ (Wrapping(ccz as i64) * z_rand) ^ Wrapping(seed);
                rand.set_seed(seed.0);

                self.structure.generate(ccx, ccz, chunk, self.range, &mut rand);

            }
        }

    }

}
