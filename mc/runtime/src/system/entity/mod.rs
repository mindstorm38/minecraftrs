use mc_core::rand::java::JavaRandom;

mod ai;
pub use ai::*;


#[derive(Debug)]
pub struct RuntimeEntity {
    /// The random instance for this specific entity, might use a non-java random later.
    rand: JavaRandom
}

impl RuntimeEntity {

    pub fn new() -> Self {
        Self {
            rand: JavaRandom::new_seeded()
        }
    }

}
