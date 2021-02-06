
#[inline(always)]
pub unsafe fn mutate_ref<T>(from: &T) -> &mut T {
    &mut *(from as *const T as *mut T)
}
