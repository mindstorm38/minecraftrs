use std::mem::ManuallyDrop;

mod version;
mod packed;
mod palette;
mod cache;
mod sync;

pub use cache::*;
pub use packed::*;
pub use palette::*;
pub use sync::*;
pub use version::*;


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
