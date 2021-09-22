use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub mod island;
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
