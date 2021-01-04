
#[inline]
pub fn lerp(factor: f64, from: f64, to: f64) -> f64 {
    from + factor * (to - from)
}

/// A generic rectangle.
///
/// Coordinate ordering: X then Z
pub struct Rect<T> {
    pub data: Vec<T>,
    pub x_size: usize,
    pub z_size: usize
}

impl<T> Rect<T> {

    pub fn new_empty() -> Self {
        Rect { data: Vec::new(), x_size: 0, z_size: 0 }
    }

    pub fn new(x_size: usize, z_size: usize, def: T) -> Self
        where T: Clone
    {
        Rect { data: vec![def; x_size * z_size], x_size, z_size }
    }

    pub fn new_default(x_size: usize, z_size: usize) -> Self
        where T: Clone + Default
    {
        Self::new(x_size, z_size, T::default())
    }

    #[inline]
    pub fn set(&mut self, x: usize, z: usize, value: T) {
        self.data.insert(x + z * self.x_size, value);
    }

    #[inline]
    pub fn get(&self, x: usize, z: usize) -> T
        where T: Copy
    {
        self.data[x + z * self.x_size]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut T {
        &mut self.data[x + z * self.x_size]
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

    pub fn new_empty() -> Self {
        Cube { data: Vec::new(), x_size: 0, y_size: 0, z_size: 0 }
    }

    pub fn new(x_size: usize, y_size: usize, z_size: usize, def: T) -> Self
        where T: Clone
    {
        Cube { data: vec![def; x_size * y_size * z_size], x_size, y_size, z_size }
    }

    pub fn new_default(x_size: usize, y_size: usize, z_size: usize) -> Self
        where T: Clone + Default
    {
        Self::new(x_size, y_size, z_size, T::default())
    }

    pub fn reset(&mut self, value: T)
        where T: Copy
    {
        for v in &mut self.data {
            *v = value;
        }
    }

    #[inline]
    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        (x * self.z_size + z) * self.y_size + y
        // (x * self.z_size * self.y_size) + (z * self.y_size) + y
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        self.data.insert(self.get_index(x, y, z), value);
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> T
        where T: Copy
    {
        self.data[self.get_index(x, y, z)]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        let idx = self.get_index(x, y, z);
        &mut self.data[idx]
    }

}