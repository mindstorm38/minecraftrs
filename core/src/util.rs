
/// Black magic used only by BlockStateBuilder.
#[inline(always)]
pub unsafe fn mutate_ref<T>(from: &T) -> &mut T {
    &mut *(from as *const T as *mut T)
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
