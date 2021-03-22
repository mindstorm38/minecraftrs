use std::sync::atomic::{AtomicU32, Ordering};


/// A static thread-safe unique 32 bits identifier generate.
pub struct StaticUidGenerator(AtomicU32);

impl StaticUidGenerator {

    pub const fn new() -> Self {
        Self(AtomicU32::new(1))
    }

    pub fn next(&self) -> u32 {
        match self.0.fetch_add(1, Ordering::Relaxed) {
            0 => panic!("Abnormal block count, the global UID overflowed (more than 4 billion)."),
            uid => uid
        }
    }

}


/// Cardinal direction used in-game.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    East,  // +X
    West,  // -X
    South, // +Z
    North, // -Z
    Up,    // +Y
    Down,  // -Y
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black
}
