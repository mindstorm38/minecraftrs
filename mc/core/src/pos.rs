

/// This is a standard structure used to exchange block positions. Coordinates
/// are signed 32-bits integers.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl BlockPos {

    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn nil() -> Self {
        Self::new(0, 0, 0)
    }

    #[inline]
    pub fn into_array(&self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn add(&self, dx: i32, dy: i32, dz: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    #[inline]
    pub fn relative(&self, dir: Direction, mul: i32) -> Self {
        let (dx, dy, dz) = dir.normal(mul);
        self.add(dx, dy, dz)
    }

}

impl Default for BlockPos {
    fn default() -> Self {
        Self::nil()
    }
}

macro_rules! impl_block_pos_relative_shortcut {
    ($func_name:ident, $direction:ident) => {
        impl BlockPos {
            #[inline]
            pub fn $func_name(&self, mul: i32) -> Self {
                self.relative(Direction::$direction, mul)
            }
        }
    };
}

impl_block_pos_relative_shortcut!(east, East);
impl_block_pos_relative_shortcut!(west, West);
impl_block_pos_relative_shortcut!(south, South);
impl_block_pos_relative_shortcut!(north, North);
impl_block_pos_relative_shortcut!(above, Up);
impl_block_pos_relative_shortcut!(below, Down);


/// This is a standard structure used to exchange block positions. Coordinates
/// are 64-bits floating point numbers.
#[derive(Debug, PartialEq, Clone)]
pub struct EntityPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl EntityPos {

    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn nil() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn into_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

}

impl Default for EntityPos {
    fn default() -> Self {
        Self::nil()
    }
}

// Conversions //

impl From<&'_ BlockPos> for EntityPos {
    fn from(pos: &'_ BlockPos) -> Self {
        Self::new(pos.x as f64, pos.y as f64, pos.z as f64)
    }
}

impl From<&'_ EntityPos> for BlockPos {
    fn from(pos: &'_ EntityPos) -> Self {
        Self::new(pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32)
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

impl Direction {

    pub fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::North => Self::South,
            Self::Up => Self::Down,
            Self::Down => Self::Up
        }
    }

    pub fn axis(self) -> Axis {
        match self {
            Self::East | Self::West => Axis::X,
            Self::South | Self::North => Axis::Z,
            Self::Up | Self::Down => Axis::Y,
        }
    }

    pub fn normal(self, mul: i32) -> (i32, i32, i32) {
        match self {
            Self::East => (mul, 0, 0),
            Self::West => (-mul, 0, 0),
            Self::South => (0, 0, mul),
            Self::North => (0, 0, -mul),
            Self::Up => (0, mul, 0),
            Self::Down => (0, -mul, 0)
        }
    }

}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z
}

impl Axis {

    pub fn directions(self) -> (Direction, Direction) {
        match self {
            Self::X => (Direction::East, Direction::West),
            Self::Y => (Direction::Up, Direction::Down),
            Self::Z => (Direction::South, Direction::North)
        }
    }

}