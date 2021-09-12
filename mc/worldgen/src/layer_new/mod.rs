use std::num::Wrapping;

use mc_core::biome::Biome;
use mc_core::math::Rect;

mod island;
pub use island::*;
mod zoom;
pub use zoom::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    /// Special state for initial vec. Must not be set by any layer.
    Uninit,
    /// For river layers if no river.
    NoRiver,
    /// For river layers, a random value in `[2;4[` is used for final river layer.
    PotentialRiver(u8),
    /// Temporary biome state, to be converted to real river biome by biomes layer.
    River,
    /// A real biome, the final state that must be returned by last layer.
    Biome(&'static Biome)
}

impl State {
    fn expect_biome(&self) -> &'static Biome {
        match self {
            State::Biome(biome) => *biome,
            _ => panic!("Expected biome state from parent compute layer.")
        }
    }
}

pub type LayerData = Rect<State>;


/// A layer trait to implement the layer generation algorithm.
pub trait Layer {
    fn seed(&mut self, seed: i64);
    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, parents: &[&mut dyn ComputeLayer]);
}


/// A trait for different types of compute layer, like `RootLayer` and `IntermediateLayer`.
/// Compute layers are actual layers that can be chained.
pub trait ComputeLayer {

    fn seed(&mut self, seed: i64);
    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData);

    fn generate_size(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> LayerData {
        let mut data = LayerData::new(x_size, z_size, State::Uninit);
        self.generate(x, z, &mut data);
        data
    }

    fn then<N>(self, layer: N) -> IntermediateLayer<Self, N>
    where
        N: Layer
    {
        IntermediateLayer {
            previous: self,
            layer
        }
    }

}


/// This type of layer is the root of the layers tree.
pub struct RootLayer<L: Layer> {
    layer: L
}

impl<L: Layer> RootLayer<L> {

    pub fn new(layer: L) -> Self {
        Self {
            layer
        }
    }

}

impl<L: Layer> ComputeLayer for RootLayer<L> {

    fn seed(&mut self, seed: i64) {
        self.layer.seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData) {
        self.layer.generate(x, z, output, &[]);
    }

}

/// The type of all layers that or not `RootLayer`.
pub struct IntermediateLayer<P: ComputeLayer, L: Layer> {
    previous: P,
    layer: L
}

impl<P: ComputeLayer, L: Layer> ComputeLayer for IntermediateLayer<P, L> {

    fn seed(&mut self, seed: i64) {
        self.previous.seed(seed);
        self.layer.seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData) {
        self.layer.generate(x, z, output, &[&mut self.previous]);
    }

}


/// A LCG RNG specific for layers.
#[derive(Debug, Clone)]
pub struct LayerRand {
    base_seed: i64,
    world_seed: Wrapping<i64>,
    chunk_seed: Wrapping<i64>
}

impl LayerRand {

    fn hash_seed(seed: &mut Wrapping<i64>, to_add: i64) {
        *seed *= *seed * Wrapping(0x5851f42d4c957f2d) + Wrapping(0x14057b7ef767814f);
        *seed += Wrapping(to_add);
    }

    pub fn new(base_seed: i64) -> LayerRand {
        LayerRand {
            base_seed: {
                let mut new_base_seed = Wrapping(base_seed);
                Self::hash_seed(&mut new_base_seed, base_seed);
                Self::hash_seed(&mut new_base_seed, base_seed);
                Self::hash_seed(&mut new_base_seed, base_seed);
                new_base_seed.0
            },
            world_seed: Wrapping(0),
            chunk_seed: Wrapping(0)
        }
    }

    pub fn get_chunk_seed(&self) -> i64 {
        self.chunk_seed.0
    }

    pub fn init_world_seed(&mut self, world_seed: i64) {
        self.world_seed = Wrapping(world_seed);
        Self::hash_seed(&mut self.world_seed, self.base_seed);
        Self::hash_seed(&mut self.world_seed, self.base_seed);
        Self::hash_seed(&mut self.world_seed, self.base_seed);
    }

    pub fn init_chunk_seed(&mut self, x: i32, z: i32) {
        self.chunk_seed = self.world_seed;
        Self::hash_seed(&mut self.chunk_seed, x as i64);
        Self::hash_seed(&mut self.chunk_seed, z as i64);
        Self::hash_seed(&mut self.chunk_seed, x as i64);
        Self::hash_seed(&mut self.chunk_seed, z as i64);
    }

    pub fn next_int(&mut self, bound: u32) -> u32 {
        let bound = bound as i64;
        let mut i = (self.chunk_seed.0 >> 24) % bound;
        // println!("val: {}, bound: {}, i: {}", self.chunk_seed.0, bound, i);
        if i < 0 {
            i += bound; // Can be replace by rem_euclid
        }
        Self::hash_seed(&mut self.chunk_seed, self.world_seed.0);
        i as u32
    }

    pub fn choose<T: Copy>(&mut self, elements: &[T]) -> T {
        elements[self.next_int(elements.len() as u32) as usize]
    }

}
