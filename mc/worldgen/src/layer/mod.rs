use std::num::Wrapping;
use std::cell::RefCell;
use std::rc::Rc;

use mc_core::biome::Biome;
use mc_core::util::Rect;

pub mod island;
pub mod smooth;
pub mod river;
pub mod biome;
pub mod zoom;
pub mod snow;


/// A work-in-progress iterative layer generator. This aims to generate cell by cell instead
/// of grid by grid. This method can be slower for complex algorithms but doesn't require
/// heap allocation, and anyway, algorithms can use a caches.
pub trait Layer {

    type Item;

    /// Init the world seed for this layer, this can be used to internally initialize a random
    /// number generator that will be used later in the `next` method when generating.
    fn seed(&mut self, seed: i64);

    fn next(&mut self, x: i32, z: i32) -> Self::Item;

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Rect<Self::Item> {
        let mut data = Vec::with_capacity(x_size * z_size);
        for z in z..(z + z_size as i32) {
            for x in x..(x + x_size as i32) {
                data.push(self.next(x, z));
            }
        }
        Rect::from_raw(data, x_size, z_size)
    }

}


#[repr(transparent)]
pub struct LayerBuilder<L: Layer>(pub L);

impl LayerBuilder<island::IslandLayer> {
    pub fn with_island(base_seed: i64) -> Self {
        Self(island::IslandLayer::new(base_seed))
    }
}

impl<B, R> LayerBuilder<biome::MixBiomeAndRiverLayer<B, R>>
where
    B: Layer<Item = &'static Biome>,
    R: Layer<Item = bool>
{
    pub fn with_biome_and_river_mix(biome_parent: B, river_parent: R) -> Self {
        Self(biome::MixBiomeAndRiverLayer::new(biome_parent, river_parent))
    }
}

impl<L: Layer> LayerBuilder<L> {

    // Zoom //

    pub fn then_zoom_fuzzy(self, base_seed: i64) -> LayerBuilder<zoom::ZoomLayer<L, true>>
    where L::Item: Copy + Eq
    {
        LayerBuilder(zoom::ZoomLayer::new_fuzzy(self.0, base_seed))
    }

    pub fn then_zoom_smart(self, base_seed: i64) -> LayerBuilder<zoom::ZoomLayer<L, false>>
    where L::Item: Copy + Eq
    {
        LayerBuilder(zoom::ZoomLayer::new_smart(self.0, base_seed))
    }

    pub fn then_zoom_voronoi(self, base_seed: i64) -> LayerBuilder<zoom::VoronoiLayer<L>>
    where L::Item: Copy {
        LayerBuilder(zoom::VoronoiLayer::new(self.0, base_seed))
    }

    // Smooth //

    pub fn then_smooth(self, base_seed: i64) -> LayerBuilder<smooth::SmoothLayer<L>>
    where L::Item: Copy + Eq {
        LayerBuilder(smooth::SmoothLayer::new(self.0, base_seed))
    }

    // Conversions //

    pub fn into_box(self) -> LayerBuilder<BoxLayer<L::Item>>
    where
        L: 'static
    {
        LayerBuilder(BoxLayer::new(self.0))
    }

    pub fn into_shared(self) -> LayerBuilder<SharedLayer<L>> {
        LayerBuilder(SharedLayer::new_single(self.0))
    }

    pub fn into_shared_split(self) -> (LayerBuilder<SharedLayer<L>>, LayerBuilder<SharedLayer<L>>) {
        let (a, b) = SharedLayer::new_split(self.0);
        (LayerBuilder(a), LayerBuilder(b))
    }

    pub fn build(self) -> L {
        self.0
    }

}

impl<L: Layer<Item = &'static Biome>> LayerBuilder<L> {

    // Island //

    pub fn then_add_island(self, base_seed: i64) -> LayerBuilder<island::AddIslandLayer<L>> {
        LayerBuilder(island::AddIslandLayer::new(self.0, base_seed))
    }

    pub fn then_add_mushroom_island(self, base_seed: i64) -> LayerBuilder<island::AddMushroomIslandLayer<L>> {
        LayerBuilder(island::AddMushroomIslandLayer::new(self.0, base_seed))
    }

    // Snow //

    pub fn then_add_snow(self, base_seed: i64) -> LayerBuilder<snow::AddSnowLayer<L>> {
        LayerBuilder(snow::AddSnowLayer::new(self.0, base_seed))
    }

    // River //

    pub fn then_init_river(self, base_seed: i64) -> LayerBuilder<river::InitRiverLayer<L>> {
        LayerBuilder(river::InitRiverLayer::new(self.0, base_seed))
    }

    // Biome //

    pub fn then_biome(self, base_seed: i64, version: (u8, u8)) -> Option<LayerBuilder<biome::BiomeLayer<L>>> {
        biome::BiomeLayer::with_version(self.0, base_seed, version)
            .map(|l| LayerBuilder(l))
    }

    pub fn then_hills(self, base_seed: i64) -> LayerBuilder<biome::HillsLayer<L>> {
        LayerBuilder(biome::HillsLayer::new(self.0, base_seed))
    }

    pub fn then_shore(self) -> LayerBuilder<biome::ShoreLayer<L>> {
        LayerBuilder(biome::ShoreLayer::new(self.0))
    }

    pub fn then_biome_river(self, base_seed: i64) -> LayerBuilder<biome::BiomeRiverLayer<L>> {
        LayerBuilder(biome::BiomeRiverLayer::new(self.0, base_seed))
    }

}

impl<L: Layer<Item = u8>> LayerBuilder<L> {

    // River //

    pub fn then_add_river(self) -> LayerBuilder<river::AddRiverLayer<L>> {
        LayerBuilder(river::AddRiverLayer::new(self.0))
    }

}


/// A `Layer` implementation that allows to get a fixed-size layer with any layer hierarchy
/// into it. The only constraint is that you must know the item type of the layer.
pub struct BoxLayer<I> {
    layer: Box<dyn Layer<Item = I>>
}

impl<I> BoxLayer<I> {
    pub fn new<L>(layer: L) -> Self
    where
        L: Layer<Item = I> + 'static
    {
        Self {
            layer: Box::new(layer)
        }
    }
}

impl<I> Layer for BoxLayer<I> {
    
    type Item = I;

    fn seed(&mut self, seed: i64) {
        self.layer.seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {
        self.layer.next(x, z)
    }

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Rect<Self::Item> {
        self.layer.next_grid(x, z, x_size, z_size)
    }
    
}


/// A common layer implementation to work with shared layers.
pub struct SharedLayer<P> {
    layer: Rc<RefCell<P>>
}

impl<P> SharedLayer<P> {

    pub fn new(layer: Rc<RefCell<P>>) -> Self {
        Self {
            layer
        }
    }

    pub fn new_single(layer: P) -> Self {
        Self::new(Rc::new(RefCell::new(layer)))
    }

    pub fn new_split(layer: P) -> (Self, Self) {
        let layer = Self::new_single(layer);
        (layer.clone(), layer)
    }

}

impl<P> Clone for SharedLayer<P> {
    fn clone(&self) -> Self {
        Self { layer: Rc::clone(&self.layer) }
    }
}

impl<P> Layer for SharedLayer<P>
where
    P: Layer
{

    type Item = P::Item;

    fn seed(&mut self, seed: i64) {
        self.layer.borrow_mut().seed(seed);
    }

    fn next(&mut self, x: i32, z: i32) -> Self::Item {
        self.layer.borrow_mut().next(x, z)
    }

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Rect<Self::Item> {
        self.layer.borrow_mut().next_grid(x, z, x_size, z_size)
    }

}


/// A hash-table based cache specific to common biome layers coordinates. This cache
/// currently works better when working with 16x16 rectangle aligned by 16 on axis.
pub struct LayerCache<T> {
    data: Vec<Option<(i32, i32, T)>>,
}

impl<T> LayerCache<T> {

    pub fn new() -> Self {
        Self {
            data: (0..256).map(|_| None).collect()
        }
    }

    pub fn clear(&mut self) {
        self.data.fill_with(|| None);
    }

    #[inline]
    fn calc_index(x: i32, z: i32) -> usize {
        (x & 0xF) as usize | (((z & 0xF) as usize) << 4)
    }

    pub fn insert(&mut self, x: i32, z: i32, item: T) {
        self.data[Self::calc_index(x, z)] = Some((x, z, item));
    }

    pub fn get(&self, x: i32, z: i32) -> Option<&T> {
        let (cx, cz, item) = self.data[Self::calc_index(x, z)].as_ref()?;
        if *cx == x && *cz == z {
            Some(item)
        } else {
            None
        }
    }

    pub fn get_or_insert<F>(&mut self, x: i32, z: i32, func: F) -> &T
    where
        F: FnOnce() -> T
    {
        match &mut self.data[Self::calc_index(x, z)] {
            Some((cx, cz, item)) => {
                if *cx != x || *cz != z {
                    *cx = x;
                    *cz = z;
                    *item = func();
                }
                &*item
            },
            none => {
                *none = Some((x, z, func()));
                &none.as_ref().unwrap().2
            }
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

    pub fn skip(&mut self) {
        Self::hash_seed(&mut self.chunk_seed, self.world_seed.0);
    }

    pub fn choose<T: Copy>(&mut self, elements: &[T]) -> T {
        elements[self.next_int(elements.len() as u32) as usize]
    }

}
