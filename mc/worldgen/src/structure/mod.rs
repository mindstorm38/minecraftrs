use mc_core::rand::JavaRandom;
use mc_core::world::chunk::Chunk;

pub mod cave;


pub trait Structure {
    fn generate(&mut self, cx: i32, cz: i32, chunk: &mut Chunk, range: i32, rand: &mut JavaRandom);
}


pub struct StructureGenerator<S: Structure> {
    rand: JavaRandom,
    range: i32,
    structure: S
}

impl<S: Structure> StructureGenerator<S> {
    pub fn new(structure: S, range: i32) -> Self {
        Self {
            rand: JavaRandom::new_blank(),
            range,
            structure
        }
    }
}
