use std::hash::{Hash, Hasher};


/// This structure can be used to store a statically bound reference `&'static T` in a
/// safe wrapper that also implements `Send` and `Sync`. These traits are implemented
/// because the content cannot be recovered after the construction, and also because
/// the pointer has a static lifetime.
///
/// This opaque pointer also implements `Hash` and `Eq` to be usable as a map key.
#[repr(transparent)]
pub struct OpaquePtr<T>(*const T);

unsafe impl<T> Send for OpaquePtr<T> {}
unsafe impl<T> Sync for OpaquePtr<T> {}

impl<T> OpaquePtr<T> {
    #[inline]
    pub const fn new(rf: &'static T) -> Self {
        Self(rf)
    }
}

impl<T> Hash for OpaquePtr<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for OpaquePtr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for OpaquePtr<T> {}
