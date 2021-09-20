use std::collections::{HashMap, VecDeque};

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

}


pub struct LayerCache<T> {
    inner: HashMap<(i32, i32), T>,
    history: VecDeque<(i32, i32)>
}

impl<T> LayerCache<T> {

    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            history: VecDeque::new()
        }
    }

    pub fn get(&self, x: i32, z: i32) -> Option<&T> {
        self.inner.get(&(x, z))
    }

    pub fn insert(&mut self, x: i32, z: i32, data: T) {
        self.inner.insert((x, z), data);
        self.history.push_back((x, z));
        if self.inner.len() > 48 {
            // SAFETY: Unwrap should be safe because the inner len equals history len, so not empty.
            self.inner.remove(&self.history.pop_front().unwrap());
        }
    }

}
