use mc_core::pos::BlockPos;

#[derive(Debug, Default)]
pub struct TurtleEntity {
    /// True if the turtle has egg.
    has_egg: bool,
    /// The position the turtle travels toward to lay its eggs after breeding.
    home_pos: BlockPos,
    /// Used for swimming to random points in water.
    travel_pos: BlockPos
}
