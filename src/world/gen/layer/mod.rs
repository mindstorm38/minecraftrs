//!
//! Module for layered biome generator (currently only adapted from 1.2.5)
//!
//! A good reference for understanding biome generation was made for
//! the C lib [`cubiomes`] : [`LayerSummary.pdf`]
//!
//! [`cubiomes`]: https://github.com/Cubitect/cubiomes
//! [`LayerSummary.pdf`]: https://github.com/Cubitect/cubiomes/blob/master/LayerSummary.pdf
//!

use std::num::Wrapping;
use std::mem;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    /// Special state for initial vec. Must not be set by any layer.
    Uninit,
    /// Temporary biome state, used to avoid string comparison, may be replaced
    /// by "ocean" biome in biomes layer. The bool is used to specify frozen ones.
    ///
    /// Values in MC: `Ocean(false) = ocean(0)`, `Ocean(true) = frozen_ocean(10)`
    #[deprecated]
    Ocean(bool),
    /// Temporary biome state, must be replaced by real biomes in biomes layer,
    /// The bool is used to specify frozen ones.
    ///
    /// Values in MC: `Land(false) = plains(1)`, `Land(true) = ice_plains(12)`
    #[deprecated]
    Land(bool),
    /// Temporary biome state, must be replaced by real mushroom biome in biomes layer.
    #[deprecated]
    MushroomIsland,
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

    /// Return true if this is a raw ocean (not frozen), id 0 in Minecraft.
    #[deprecated]
    pub fn is_ocean(self) -> bool {
        matches!(self, State::Ocean(false))
    }

    pub fn expect_biome(self) -> u8 {
        match self {
            State::Biome(biome) => biome,
            _ => panic!("State {:?} must be a biome.", self)
        }
    }

}


pub struct LayerData {
    pub data: Vec<State>,
    pub x_size: usize,
    pub z_size: usize
}

impl LayerData {

    pub fn new(x_size: usize, z_size: usize) -> LayerData {
        LayerData {
            data: vec![State::Uninit; x_size * z_size],
            x_size,
            z_size
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, z: usize, value: State) {
        self.data.insert(x + z * self.x_size, value);
    }

    #[inline]
    pub fn get(&self, x: usize, z: usize) -> State {
        self.data[x + z * self.x_size]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut State {
        &mut self.data[x + z * self.x_size]
    }

}


/// Layer LCG pRNG
#[derive(Debug)]
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

    #[inline]
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
pub struct LayerInternal {
    pub rand: LayerRand,
    pub parent: Option<Box<Layer>>,
    // Used for river mix with biomes, not using vec for parents, because its sizeof will
    // be 24, but these two options are 16 byte.
    pub parent_aux: Option<Box<Layer>>
}

/// A layer handler used to compute this layer data, may call the parent layer.
pub type LayerHandlerFn = fn(x: i32, z: i32, output: &mut LayerData, internal: &mut LayerInternal);

/// An effective layer composed of an internal mutable data and a handler.
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
        let mut data = LayerData::new(x_size, z_size);
        self.inner_generate(x, z, &mut data);
        data
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


fn common_layer() -> Layer {
    let layer = Layer::new_island(1);
    let layer = Layer::new_fuzzy_zoom(2000, layer);
    let layer = Layer::new_add_island(1, layer);
    let layer = Layer::new_zoom(2001, layer);
    let layer = Layer::new_add_island(2, layer);
    let layer = Layer::new_add_snow(2, layer);
    let layer = Layer::new_zoom(2002, layer);
    let layer = Layer::new_add_island(3, layer);
    let layer = Layer::new_zoom(2003, layer);
    let layer = Layer::new_add_island(4, layer);
    let layer = Layer::new_add_mushroom_island(5, layer);
    layer
}

pub fn test() {

    let mut river = Layer::new_river_init(100, common_layer());
    river = Layer::new_zoom_multiple(1000, river, 6);
    river = Layer::new_river(1, river);
    river = Layer::new_smooth(1000, river);

    let mut biome = Layer::new_biome_1_2(200, common_layer());
    biome = Layer::new_zoom_multiple(1000, biome, 2);
    biome = Layer::new_hills(1000, biome);
    for i in 0..4 {
        biome = Layer::new_zoom(1000 + i, biome);
        match i {
            0 => biome = Layer::new_add_island(3, biome),
            1 => {
                biome = Layer::new_shore(1000, biome);
                biome = Layer::new_biome_rivers(1000, biome);
            },
            _ => {}
        }
    }
    biome = Layer::new_smooth(1000, biome);

    let all = Layer::new_mix_biome_river(100, biome, river);

    // TODO: Voronoi Zoom

    //let layer = Layer::new(1, island);
    //let layer = Layer::with_parent(layer);
    //let layer = Layer::with_parent(layer);

}