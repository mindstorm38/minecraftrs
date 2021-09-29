use std::ops::AddAssign;

/// A generic rectangle.
/// Coordinates order: X then Z
pub struct Rect<T> {
    pub data: Vec<T>,
    pub x_size: usize,
    pub z_size: usize
}

impl<T> Rect<T> {

    pub fn new(x_size: usize, z_size: usize, def: T) -> Self
        where
            T: Clone
    {
        Rect { data: vec![def; x_size * z_size], x_size, z_size }
    }

    pub fn new_default(x_size: usize, z_size: usize) -> Self
        where
            T: Clone + Default
    {
        Self::new(x_size, z_size, T::default())
    }

    pub fn from_raw(data: Vec<T>, x_size: usize, z_size: usize) -> Self {
        assert_eq!(data.len(), x_size * z_size);
        Self {
            data,
            x_size,
            z_size
        }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Clone
    {
        for v in &mut self.data {
            *v = value.clone();
        }
    }

    #[inline]
    fn get_index(&self, x: usize, z: usize) -> usize {
        debug_assert!(x < self.x_size && z < self.z_size);
        x + (z * self.x_size)
    }

    #[inline]
    pub fn set(&mut self, x: usize, z: usize, value: T) {
        let idx = self.get_index(x, z);
        self.data[idx] = value;
    }

    #[inline]
    pub fn add(&mut self, x: usize, z: usize, value: T)
    where
        T: AddAssign
    {
        let idx = self.get_index(x, z);
        self.data[idx] += value;
    }

    #[inline]
    pub fn get(&self, x: usize, z: usize) -> &T {
        let idx = self.get_index(x, z);
        &self.data[idx]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut T {
        let idx = self.get_index(x, z);
        &mut self.data[idx]
    }

}

/// A generic cube.
///
/// Coordinate ordering: Y then Z then X
pub struct Cube<T> {
    pub data: Vec<T>,
    pub x_size: usize,
    pub y_size: usize,
    pub z_size: usize
}

impl<T> Cube<T> {

    pub fn new(x_size: usize, y_size: usize, z_size: usize, def: T) -> Self
        where
            T: Clone
    {
        Cube { data: vec![def; x_size * y_size * z_size], x_size, y_size, z_size }
    }

    pub fn new_default(x_size: usize, y_size: usize, z_size: usize) -> Self
        where
            T: Clone + Default
    {
        Self::new(x_size, y_size, z_size, T::default())
    }

    pub fn from_raw(data: Vec<T>, x_size: usize, y_size: usize, z_size: usize) -> Self {
        assert_eq!(data.len(), x_size * y_size * z_size);
        Self {
            data,
            x_size,
            y_size,
            z_size
        }
    }

    pub fn fill(&mut self, value: T)
        where
            T: Clone
    {
        for v in &mut self.data {
            *v = value.clone();
        }
    }

    #[inline]
    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        debug_assert!(x < self.x_size && y < self.y_size && z < self.z_size);
        (x * self.z_size + z) * self.y_size + y
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        let idx = self.get_index(x, y, z);
        self.data[idx] = value;
    }

    #[inline]
    pub fn add(&mut self, x: usize, y: usize, z: usize, value: T)
    where
        T: AddAssign
    {
        let idx = self.get_index(x, y, z);
        self.data[idx] += value;
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> &T {
        let idx = self.get_index(x, y, z);
        &self.data[idx]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        let idx = self.get_index(x, y, z);
        &mut self.data[idx]
    }

}
