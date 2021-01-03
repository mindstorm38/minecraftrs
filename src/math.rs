
#[inline]
pub fn lerp(factor: f64, from: f64, to: f64) -> f64 {
    from + factor * (to - from)
}
