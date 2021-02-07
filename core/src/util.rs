
/// Black magic used only by BlockStateBuilder.
#[inline(always)]
pub unsafe fn mutate_ref<T>(from: &T) -> &mut T {
    &mut *(from as *const T as *mut T)
}


/// Cardinal direction used in-game.
#[derive(strum::ToString, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    #[strum(serialize = "east")]
    East,  // +X
    #[strum(serialize = "west")]
    West,  // -X
    #[strum(serialize = "south")]
    South, // +Z
    #[strum(serialize = "north")]
    North, // -Z
    #[strum(serialize = "up")]
    Up,    // +Y
    #[strum(serialize = "down")]
    Down,  // -Y
}


#[derive(strum::ToString, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    #[strum(serialize = "x")]
    X,
    #[strum(serialize = "y")]
    Y,
    #[strum(serialize = "z")]
    Z
}
