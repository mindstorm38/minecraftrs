use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem::MaybeUninit;

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

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Vec<Self::Item> {
        let mut data = Vec::with_capacity(x_size * z_size);
        for z in z..(z + z_size as i32) {
            for x in x..(x + x_size as i32) {
                data.push(self.next(x, z));
            }
        }
        data
    }

    // Extensions

    fn add_island(self, base_seed: i64) -> island::AddIslandLayer<Self>
    where
        Self: Sized
    {
        island::AddIslandLayer::new(self, base_seed)
    }

    fn zoom_fuzzy(self, base_seed: i64) -> zoom::ZoomLayer<Self, true>
    where
        Self: Sized
    {
        zoom::ZoomLayer::new_fuzzy(self, base_seed)
    }

    fn zoom_smart(self, base_seed: i64) -> zoom::ZoomLayer<Self, false>
    where
        Self: Sized
    {
        zoom::ZoomLayer::new_smart(self, base_seed)
    }

    fn add_snow(self, base_seed: i64) -> snow::AddSnowLayer<Self>
    where
        Self: Sized
    {
        snow::AddSnowLayer::new(self, base_seed)
    }

    fn add_mushroom_island(self, base_seed: i64) -> island::AddMushroomIsland<Self>
    where
        Self: Sized
    {
        island::AddMushroomIsland::new(self, base_seed)
    }

    fn init_river(self, base_seed: i64) -> river::InitRiverLayer<Self>
    where
        Self: Sized
    {
        river::InitRiverLayer::new(self, base_seed)
    }

    fn add_river(self) -> river::AddRiverLayer<Self>
    where
        Self: Sized
    {
        river::AddRiverLayer::new(self)
    }

    fn smooth(self, base_seed: i64) -> smooth::SmoothLayer<Self>
    where
        Self: Sized
    {
        smooth::SmoothLayer::new(self, base_seed)
    }

    fn biome(self, base_seed: i64, version: (u8, u8)) -> Option<biome::BiomeLayer<Self>>
    where
        Self: Sized
    {
        biome::BiomeLayer::from_version(self, base_seed, version)
    }

    fn into_box(self) -> BoxLayer<Self::Item>
    where
        Self: Sized + 'static
    {
        BoxLayer::new(self)
    }

    fn into_shared(self) -> SharedLayer<Self>
    where
        Self: Sized
    {
        SharedLayer::new_single(self)
    }

    fn into_shared_split(self) -> (SharedLayer<Self>, SharedLayer<Self>)
    where
        Self: Sized
    {
        SharedLayer::new_split(self)
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

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Vec<Self::Item> {
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

    fn next_grid(&mut self, x: i32, z: i32, x_size: usize, z_size: usize) -> Vec<Self::Item> {
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
