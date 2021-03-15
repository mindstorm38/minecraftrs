use std::ops::{Deref, DerefMut};


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


/// This structure holds a reference together with its owner.
/// This is useful to return a safe reference to an owner that
/// is normally temporary. This structure implements Deref to
/// return the internal reference.
pub struct OwnerRef<O, V> {
    #[allow(dead_code)]
    owner: O,
    ptr: *const V
}

impl<O, V> OwnerRef<O, V> {
    pub fn new_unchecked(owner: O, ptr: *const V) -> OwnerRef<O, V> {
        OwnerRef { owner, ptr }
    }
}

impl<O, V> Deref for OwnerRef<O, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}


pub struct OwnerMut<O, V> {
    #[allow(dead_code)]
    owner: O,
    ptr: *mut V
}

impl<O, V> OwnerMut<O, V> {
    pub fn new_unchecked(owner: O, ptr: *mut V) -> OwnerMut<O, V> {
        OwnerMut { owner, ptr }
    }
}

impl<O, V> Deref for OwnerMut<O, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<O, V> DerefMut for OwnerMut<O, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
