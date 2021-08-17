use std::sync::atomic::{AtomicU32, Ordering};
use std::mem::ManuallyDrop;

mod generic;
mod version;
mod packed;
mod palette;
mod cache;

pub use generic::*;
pub use version::*;
pub use packed::*;
pub use palette::*;
pub use cache::*;


/// A static thread-safe unique 32 bits identifier generate.
/// This structure is made for static constants and will be
/// interior mutated when retrieving the next UID.
///
/// The actual maximum of different UIDs is <code>2<sup>32</sup>-1</code>
/// because of the sentinel value `0` for the overflow detection.
pub struct UidGenerator(AtomicU32);

impl UidGenerator {

    /// Create a new static UID generator, this method can be called in
    /// to define a static constant.
    /// Example:
    /// ```
    /// use mc_core::util::UidGenerator;
    /// static UID: UidGenerator = UidGenerator::new();
    /// ```
    pub const fn new() -> Self {
        Self(AtomicU32::new(1))
    }

    /// Get the next UID, thread-safely. If the UID overflows the maximum
    /// UID <code>2<sup>32</sup>-1</code>, the function panics.
    pub fn next(&self) -> u32 {
        match self.0.fetch_add(1, Ordering::Relaxed) {
            0 => panic!("Abnormal UID count: overflowed (more than 4 billion)."),
            uid => uid
        }
    }

}


pub unsafe fn cast_vec<Src, Dst>(src: Vec<Src>) -> Vec<Dst> {
    debug_assert_eq!(std::mem::size_of::<Src>(), std::mem::size_of::<Dst>());
    debug_assert_eq!(std::mem::align_of::<Src>(), std::mem::align_of::<Dst>());
    let mut src = ManuallyDrop::new(src);
    let (ptr, len, cap) = (src.as_mut_ptr(), src.len(), src.capacity());
    Vec::from_raw_parts(ptr as *mut Dst, len, cap)
}

pub fn cast_vec_ref_to_ptr<T>(src: Vec<&'static T>) -> Vec<*const T> {
    unsafe { cast_vec(src) }
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


#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            print!("[{}:{}] ", file!(), line!());
            println!($($arg)*);
        }
        #[cfg(not(debug_assertions))]
        {}
    };
}
