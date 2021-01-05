//!
//! Module for layered biome generator (currently only adapted from 1.2.5)
//!
//! A good reference for understanding biome generation was made for
//! the C lib [`cubiomes`] : [`LayerSummary.pdf`]
//!
//! [`cubiomes`]: https://github.com/Cubitect/cubiomes
//! [`LayerSummary.pdf`]: https://github.com/Cubitect/cubiomes/blob/master/LayerSummary.pdf
//!

use crate::math::Rect;
use crate::biome::{Biome, BiomeRegistry};
use std::num::Wrapping;


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
    Biome(u8)
}

impl State {
    pub fn expect_biome(self) -> u8 {
        match self {
            State::Biome(biome) => biome,
            _ => panic!("State {:?} must be a biome.", self)
        }
    }
}


pub type LayerData = Rect<State>;
pub type BiomeRect<'a> = Rect<&'a Biome>;


impl LayerData {
    pub fn debug(&self, title: &'static str) {
        println!(" -----------------------");
        println!("{} ({}x{})", title, self.x_size, self.z_size);
        for z in 0..self.z_size {
            for x in 0..self.x_size {
                print!("{} ", match self.get(x, z) {
                    State::Uninit => ' ',
                    State::NoRiver => 'N',
                    State::PotentialRiver(_) => 'P',
                    State::River => 'R',
                    State::Biome(b) => b.to_string().chars().next().unwrap()
                });
            }
            println!();
        }
        println!(" -----------------------");
    }
}


pub fn build_biome_rect(input: LayerData, registry: &BiomeRegistry) -> BiomeRect {

    let mut output = BiomeRect::new_empty();

    output.x_size = input.x_size;
    output.z_size = input.z_size;

    for state in input.data {
        let biome = state.expect_biome();
        if let Some(t) = registry.get_from_id(biome) {
            output.data.push(t);
        } else {
            panic!("Failed to find a biome for the id {} in the registry.", biome);
        }
    }

    output

}


/// Layer LCG pRNG
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
        if i < 0 {
            i += bound;
        }
        Self::hash_seed(&mut self.chunk_seed, self.world_seed.0);
        i as u32
    }

    pub fn choose<'a, T>(&mut self, elements: &'a [T]) -> &'a T {
        &elements[self.next_int(elements.len() as u32) as usize]
    }

    pub fn choose_copy<T: Copy>(&mut self, elements: &[T]) -> T {
        elements[self.next_int(elements.len() as u32) as usize]
    }

    pub fn choose_ref<'a, T>(&mut self, elements: &'a [&T]) -> &'a T {
        *self.choose(elements)
    }

    pub fn choose_ref_and_clone<T: Clone>(&mut self, elements: &[&T]) -> T {
        (*self.choose(elements)).clone()
    }

}


/// Internal mutable data for layers, to be passed to the handler of `Layer`.
#[derive(Clone)]
pub struct LayerInternal {
    pub rand: LayerRand,
    pub parent: Option<Box<Layer>>,
    // Used for river mix with biomes, not using vec for parents, because its sizeof will
    // be 24, but these two options are 16 bytes wide.
    pub parent_aux: Option<Box<Layer>>
}

/// A layer handler used to compute this layer data, may call the parent layer.
pub type LayerHandlerFn = fn(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal);

/// An effective layer composed of an internal mutable data and a handler.
#[derive(Clone)]
pub struct Layer {
    internal: LayerInternal,
    handler: LayerHandlerFn
}

impl Layer {

    #[inline]
    pub fn new_child(base_seed: i64, handler: LayerHandlerFn, parent: Layer) -> Layer {
        Self::new(base_seed, handler, Some(Box::new(parent)), None)
    }

    #[inline]
    pub fn new_orphan(base_seed: i64, handler: LayerHandlerFn) -> Layer {
        Self::new(base_seed, handler, None, None)
    }

    #[inline]
    fn new(base_seed: i64, handler: LayerHandlerFn, parent: Option<Box<Layer>>, parent_aux: Option<Box<Layer>>) -> Layer {
        Layer {
            internal: LayerInternal {
                rand: LayerRand::new(base_seed),
                parent,
                parent_aux
            },
            handler
        }
    }

    pub fn init_world_seed(&mut self, world_seed: i64) {
        self.internal.rand.init_world_seed(world_seed);
        if let Some(parent) = &mut self.internal.parent {
            parent.init_world_seed(world_seed)
        }
        if let Some(parent_aux) = &mut self.internal.parent_aux {
            parent_aux.init_world_seed(world_seed);
        }
    }

    pub fn inner_generate(&mut self, x: i32, z: i32, output: &mut LayerData) {
        (self.handler)(x, z, output, &mut self.internal);
    }

    pub fn generate(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> LayerData {
        let mut data = LayerData::new(x_size, z_size, State::Uninit);
        self.inner_generate(x, z, &mut data);
        data
    }

    pub fn expect_parent(&mut self) -> &mut Layer {
        self.internal.expect_parent()
    }

}

impl LayerInternal {

    pub fn expect_parent(&mut self) -> &mut Layer {
        self.parent.as_mut().unwrap()
    }

    pub fn expect_parent_aux(&mut self) -> &mut Layer {
        self.parent_aux.as_mut().expect("This layer has now auxiliary parent.")
    }

}

/// Common macro to automatically implement a constructor for
/// a specific layer type.
macro_rules! impl_layer {
    ($func:ident, $new_func:ident) => {
        impl crate::world::gen::layer::Layer {
            #[inline]
            pub fn $new_func(base_seed: i64, parent: Self) -> Self {
                Self::new_child(base_seed, $func, parent)
            }
        }
    };
    (orphan $func:ident, $new_func:ident) => {
        impl crate::world::gen::layer::Layer {
            #[inline]
            pub fn $new_func(base_seed: i64) -> Self {
                Self::new_orphan(base_seed, $func)
            }
        }
    };
}


pub mod zoom;
pub mod island;
pub mod snow;
pub mod river;
pub mod smooth;
pub mod biome;
pub mod voronoi;
