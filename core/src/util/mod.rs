use std::sync::atomic::{AtomicU32, Ordering};
use std::mem::ManuallyDrop;
use std::hash::{Hash, Hasher};

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


/// A transparent pointer type used to store references with static lifetime as a
/// raw constant pointer. This is used internally as a pointer key for hash maps.
/// This type implements `Send + Sync` because this type of pointer is always valid.
#[repr(transparent)]
pub struct StaticPtr<T>(pub *const T);
unsafe impl<T> Send for StaticPtr<T> {}
unsafe impl<T> Sync for StaticPtr<T> {}

impl<T> Hash for StaticPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for StaticPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for StaticPtr<T> {}


/// A macro used internally to MinecraftRS to count the number of tokens you give
/// to the macro and return the count as usize.
#[macro_export]
macro_rules! count {
    () => (0usize);
    ($x0:tt $x1:tt $x2:tt $x3:tt $x4:tt $x5:tt $x6:tt $x7:tt
     $x8:tt $x9:tt $x10:tt $x11:tt $x12:tt $x13:tt $x14:tt $x15:tt $($xs:tt)*) => (16usize + $crate::count!($($xs)*));
    ($x0:tt $x1:tt $x2:tt $x3:tt $x4:tt $x5:tt $x6:tt $x7:tt $($xs:tt)*) => (8usize + $crate::count!($($xs)*));
    ($x0:tt $x1:tt $x2:tt $x3:tt $($xs:tt)*) => (4usize + $crate::count!($($xs)*));
    ($x0:tt $x1:tt $($xs:tt)*) => (2usize + $crate::count!($($xs)*));
    ($x0:tt $($xs:tt)*) => (1usize + $crate::count!($($xs)*));
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
