use mc_core::world::chunk::ChunkGuard;
use mc_core::rand::JavaRandom;


/// A trait to implement on feature distribution structures to use later in composed structures.
pub trait Distrib {
    fn pick_pos(&self, chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> Option<(i32, i32, i32)>;
}


pub struct UniformVerticalDistrib {
    y_min: u32,
    y_max: u32
}

impl UniformVerticalDistrib {
    pub fn new(y_min: u32, y_max: u32) -> Self {
        UniformVerticalDistrib { y_min, y_max }
    }
}

impl Distrib for UniformVerticalDistrib {
    fn pick_pos(&self, _chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let ry = rand.next_int_bounded((self.y_max - self.y_min) as i32) + self.y_min as i32;
        let rz = z + rand.next_int_bounded(16);
        Some((rx, ry, rz))
    }
}


pub struct TriangularVerticalDistrib {
    y_center: u32,
    y_spread: u32
}

impl TriangularVerticalDistrib {
    pub fn new(y_center: u32, y_spread: u32) -> Self {
        TriangularVerticalDistrib { y_center, y_spread }
    }
}

impl Distrib for TriangularVerticalDistrib {
    fn pick_pos(&self, _chunk: &mut ChunkGuard, rand: &mut JavaRandom, x: i32, _y: i32, z: i32) -> Option<(i32, i32, i32)> {
        let rx = x + rand.next_int_bounded(16);
        let ry = rand.next_int_bounded(self.y_spread as i32) + rand.next_int_bounded(self.y_spread as i32) + (self.y_center - self.y_spread) as i32;
        let rz = z + rand.next_int_bounded(16);
        Some((rx, ry, rz))
    }
}
