

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
#[repr(u8)]
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

impl DyeColor {
    pub fn get_index(self) -> u8 {
        unsafe { std::mem::transmute(self) }
    }
}