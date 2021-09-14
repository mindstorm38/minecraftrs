use std::num::Wrapping;
use std::cell::RefCell;

use mc_core::biome::Biome;
use mc_core::math::Rect;

mod island;
mod zoom;
mod snow;
mod river;

pub use island::*;
pub use zoom::*;
pub use snow::*;
pub use river::*;



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

    /// A method called to set the world's seed that will be used to generate biomes.
    fn seed(&mut self, seed: i64);

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, parents: &mut [&mut dyn ComputeLayer]);

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

}


/// This structure store a layer with some additional information in order to
/// speed up generation and improve API.
struct LayerStorage {
    /// The storage of the layer.
    layer: Box<dyn NewLayer>,
    /// A void pointer to the layer, it is only used to call generate function.
    layer_ptr: *mut (),
    /// The generate method pointer, stored in this method in order to avoid one
    /// step of dynamic dispatching.
    generate_func: fn(*mut (), i32, i32, &mut LayerData, LayerContext),
    /// Optional indices of parents, these indices will be used when calling
    /// `LayerContext::generate_parent`. If there is `None` parent indices or
    /// the parent index is not in the range of the array, the parent index is
    /// calculated from the index of the calling layer minus the given parent
    /// index.
    parent_indices: Option<Vec<usize>>
}

// We enforce send because the 'layer_ptr' should be valid as long as the object is not dropped.
unsafe impl Send for LayerStorage {}

pub struct LayerSystem {
    layers: Vec<RefCell<LayerStorage>>,
}

impl LayerSystem {

    pub fn new() -> Self {
        Self {
            layers: Vec::new()
        }
    }

    pub fn push_layer_with_parents<L>(&mut self, layer: L, parent_indices: Option<Vec<usize>>) -> usize
    where
        L: NewLayer + 'static
    {
        let boxed_layer = Box::new(layer);
        let layer_ptr = unsafe { boxed_layer.as_ref() as *const L as *mut () };
        let generate_func: fn(&mut L, i32, i32, &mut LayerData, LayerContext) = L::generate;
        let push_index = self.layers.len();
        self.layers.push(RefCell::new(LayerStorage {
            layer: boxed_layer,
            layer_ptr,
            generate_func: unsafe { std::mem::transmute(generate_func) },
            parent_indices
        }));
        push_index
    }

    #[inline]
    pub fn push_layer<L>(&mut self, layer: L) -> usize
    where
        L: NewLayer + 'static
    {
        self.push_layer_with_parents(layer, None)
    }

    pub fn seed(&mut self, seed: i64) {
        for layer in &self.layers {
            layer.borrow_mut().layer.seed(seed);
        }
    }

    pub fn generate(&self, index: usize, x: i32, z: i32, output: &mut LayerData) -> bool {

        if index < self.layers.len() {
            return false;
        }

        // Here we don't care of the boxed dynamic layer because we want to use our own
        // dynamic dispatching that take one indirection less.
        let storage = self.layers[index].borrow_mut();

        let ctx = LayerContext {
            system: self,
            layer_index: index,
            parent_indices: storage.parent_indices
                .as_ref()
                .map(|v| v.as_slice())
                .unwrap_or(&[])
        };

        (storage.generate_func)(storage.layer_ptr, x, z, output, ctx);
        true

    }

    pub fn generate_size(&self, index: usize, x: i32, z: i32, x_size: usize, z_size: usize) -> Option<LayerData> {
        let mut data = LayerData::new(x_size, z_size, State::Uninit);
        if self.generate(index, x, z, &mut data) {
            Some(data)
        } else {
            None
        }
    }

    pub fn generate_root(&self, x: i32, z: i32, x_size: usize, z_size: usize) -> LayerData {
        assert!(!self.layers.is_empty(), "This layer system has no layer to generate.");
        // SAFETY: We can unwrap because we assert that there is at least a layer.
        self.generate_size(self.layers.len() - 1, x, z, x_size, z_size).unwrap()
    }

}

pub struct LayerContext<'a> {
    system: &'a LayerSystem,
    layer_index: usize,
    parent_indices: &'a [usize]
}

impl<'a> LayerContext<'a> {

    /// Internal method to compute the real parent index in the layers vec. In first place we
    /// check if the user has given a parent index that is an alias, if it's not the case we
    /// get subtract the parent index plus 1 to the layer (the caller) index.
    #[inline]
    fn get_parent_index(&self, parent_index: usize) -> usize {
        self.parent_indices
            .get(parent_index)
            .copied()
            .unwrap_or(self.layer_index - 1 - parent_index)
    }

    pub fn generate(&self, parent_index: usize, x: i32, z: i32, output: &mut LayerData) -> bool {
        self.system.generate(self.get_parent_index(parent_index), x, z, output)
    }

    pub fn generate_size(&self, parent_index: usize, x: i32, z: i32, x_size: usize, z_size: usize) -> Option<LayerData> {
        self.system.generate_size(self.get_parent_index(parent_index), x, z, x_size, z_size)
    }

}

pub trait NewLayer {
    fn seed(&mut self, seed: i64);
    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, context: LayerContext);
}


/// This type of layer is the root of the layers tree.
pub struct RootLayer<L> {
    layer: L
}

impl<L> RootLayer<L>
where
    L: Layer
{

    pub fn new(layer: L) -> Self {
        Self {
            layer
        }
    }

    pub fn then<N>(self, layer: N) -> IntermediateLayer<Self, N>
    where
        N: Layer
    {
        IntermediateLayer {
            previous: self,
            layer
        }
    }

}

impl<L> ComputeLayer for RootLayer<L>
where
    L: Layer
{

    fn seed(&mut self, seed: i64) {
        self.layer.seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData) {
        self.layer.generate(x, z, output, &mut []);
    }

}

impl<L> Clone for RootLayer<L>
    where
        L: Layer + Clone
{
    fn clone(&self) -> Self {
        Self {
            layer: self.layer.clone()
        }
    }
}

/// The type of all layers that are created from `RootLayer` or `Self`.
pub struct IntermediateLayer<P, L> {
    previous: P,
    layer: L
}

impl<P, L> IntermediateLayer<P, L>
where
    P: ComputeLayer,
    L: Layer
{

    pub fn then<N>(self, layer: N) -> IntermediateLayer<Self, N>
    where
        N: Layer
    {
        IntermediateLayer {
            previous: self,
            layer
        }
    }

}

impl<P, L> ComputeLayer for IntermediateLayer<P, L>
where
    P: ComputeLayer,
    L: Layer
{

    fn seed(&mut self, seed: i64) {
        self.previous.seed(seed);
        self.layer.seed(seed);
    }

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData) {
        self.layer.generate(x, z, output, &mut [&mut self.previous]);
    }

}

impl<P, L> Clone for IntermediateLayer<P, L>
where
    P: ComputeLayer + Clone,
    L: Layer + Clone
{
    fn clone(&self) -> Self {
        Self {
            previous: self.previous.clone(),
            layer: self.layer.clone()
        }
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
