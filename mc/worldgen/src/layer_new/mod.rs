use std::cell::{RefCell, RefMut};
use std::num::Wrapping;

use mc_core::biome::Biome;
use mc_core::math::Rect;
use std::fmt::{Debug, Formatter, Write};

pub mod island;
pub mod zoom;
pub mod snow;
pub mod river;
pub mod smooth;
pub mod biome;
pub mod voronoi;


#[derive(Copy, Clone, Eq)]
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
            _ => panic!("Expected biome state from parent layer.")
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Uninit => f.write_str("[]"),
            State::NoRiver => f.write_str("NR"),
            State::PotentialRiver(p) => f.write_fmt(format_args!("R{}", *p)),
            State::River => f.write_str("R "),
            State::Biome(biome) => f.write_fmt(format_args!("{:02}", biome.get_id()))
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Biome(b0), State::Biome(b1)) => (*b0) == (*b1),
            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b)
        }
    }
}

/// Type alias for a layer data.
pub type LayerData = Rect<State>;

pub fn debug_layer_data(data: &LayerData) {

    print!("   ");
    for x in 0..data.x_size {
        print!("{:02} ", x);
    }
    println!();

    for z in 0..data.z_size {
        print!("{:02} ", z);
        for x in 0..data.x_size {
            print!("{:?} ", data.get_ref(x, z));
        }
        println!();
    }

}

/*pub fn layer_into_biomes<const LEN: usize>(data: LayerData) -> Option<[&'static Biome; LEN]> {
    if data.data.len() == LEN {
        let mut arr: [MaybeUninit<&'static Biome>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut state_it = data.data.into_iter();
        for uninit_biome in &mut arr {
            *uninit_biome = MaybeUninit::new(state_it.next().unwrap().expect_biome());
        }
        Some(unsafe { std::mem::transmute(arr) })
    } else {
        None
    }
}*/


/// A layer trait to implement the layer generation algorithm.
pub trait Layer {

    /// A method called to set the world's seed that will be used to generate biomes.
    fn seed(&mut self, seed: i64);

    fn generate(&mut self, x: i32, z: i32, output: &mut LayerData, ctx: LayerContext);

}


/// This structure store a layer with some additional information in order to
/// speed up generation and improve API.
struct LayerStorage {
    /// The storage of the layer.
    layer: Box<dyn Layer>,
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
    parent_indices: Vec<usize>,
    /// Debug name of the layer type.
    layer_type_name: &'static str
}

// We enforce send because the 'layer_ptr' should be valid as long as the object is not dropped.
unsafe impl Send for LayerStorage {}


/// The layer system is a linear sequence storage of layers.
pub struct LayerSystem {
    layers: Vec<RefCell<LayerStorage>>,
}

impl LayerSystem {

    pub fn new() -> Self {
        Self {
            layers: Vec::new()
        }
    }

    pub fn push_with_parents<L>(&mut self, layer: L, parent_indices: Vec<usize>)
    where
        L: Layer + 'static
    {
        let layer = Box::new(layer);
        let layer_ptr = layer.as_ref() as *const L as *mut ();
        let generate_func: fn(&mut L, i32, i32, &mut LayerData, LayerContext) = L::generate;
        self.layers.push(RefCell::new(LayerStorage {
            layer,
            layer_ptr,
            generate_func: unsafe { std::mem::transmute(generate_func) },
            parent_indices,
            layer_type_name: std::any::type_name::<L>()
        }));
    }

    #[inline]
    pub fn push<L>(&mut self, layer: L)
    where
        L: Layer + 'static
    {
        self.push_with_parents(layer, Vec::new())
    }

    #[inline]
    pub fn push_iter<I, L>(&mut self, layers_it: I)
    where
        I: Iterator<Item = L>,
        L: Layer + 'static
    {
        for layer in layers_it {
            self.push(layer);
        }
    }

    /// Returns the last layer index, if there is no layer, `None` is returned.
    pub fn last_index(&self) -> Option<usize> {
        if self.layers.is_empty() { None } else { Some(self.layers.len() - 1) }
    }

    /// Seed all layers.
    pub fn seed(&mut self, seed: i64) {
        for layer in &self.layers {
            layer.borrow_mut().layer.seed(seed);
        }
    }

    /// Try to borrow a layer.
    pub fn borrow_layer(&self, index: usize) -> Option<LayerRef> {
        self.layers.get(index)?.try_borrow_mut().ok().map(move |layer| {
            LayerRef {
                layer,
                system: self,
                layer_index: index
            }
        })
    }

    /// Try to borrow the root layer (the last added layer).
    pub fn borrow_root(&self) -> Option<LayerRef> {
        if self.layers.is_empty() {
            None
        } else {
            self.borrow_layer(self.layers.len() - 1)
        }
    }

}

impl Debug for LayerSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, layer) in self.layers.iter().enumerate() {
            match layer.try_borrow() {
                Ok(layer) => {
                    f.write_fmt(format_args!("#{:02} {}", i, layer.layer_type_name))?;
                    if !layer.parent_indices.is_empty() {
                        f.write_str(" parents: ")?;
                        f.debug_list()
                            .entries(layer.parent_indices.iter())
                            .finish()?;
                    }
                    f.write_char('\n')?;
                },
                Err(_) => f.write_str("- already mutably borrowed")?
            }
        }
        Ok(())
    }
}

/// A rich reference to a layer, returned when borrowing a layer. This structure keep the
/// `RefMut` from the ref cell, this allows only on ref to the same layer at the same time.
/// Once you the this ref, you can start generating this layer.
pub struct LayerRef<'a> {
    layer: RefMut<'a, LayerStorage>,
    system: &'a LayerSystem,
    layer_index: usize,
}

impl<'a> LayerRef<'a> {

    pub fn generate(&self, x: i32, z: i32, output: &mut LayerData) {
        (self.layer.generate_func)(self.layer.layer_ptr, x, z, output, LayerContext {
            system: self.system,
            layer_index: self.layer_index,
            parent_indices: &self.layer.parent_indices[..]
        });
    }

    pub fn generate_size(&self, x: i32, z: i32, x_size: usize, z_size: usize) -> LayerData {
        let mut data = LayerData::new(x_size, z_size, State::Uninit);
        self.generate(x, z, &mut data);
        data
    }

}

/// This is a temporary context given to the generate method of implementations of `Layer`.
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

    pub fn borrow_parent(&self, parent_index: usize) -> Option<LayerRef> {
        self.system.borrow_layer(self.get_parent_index(parent_index))
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
