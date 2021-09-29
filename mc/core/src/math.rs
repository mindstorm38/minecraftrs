
#[allow(clippy::approx_constant)]
pub const JAVA_PI: f64 = 3.14159265358979323846;


#[inline]
pub fn lerp(factor: f64, from: f64, to: f64) -> f64 {
    from + factor * (to - from)
}


#[inline(always)]
fn mc_sin_table(i: u16) -> f32 {
    (i as f64 * JAVA_PI * 2.0 / 65536.0).sin() as f32
}


/// Emulate the Minecraft sinus lookup table.
pub fn mc_sin(x: f32) -> f32 {
    mc_sin_table(((x * 10430.38) as i32 & 0xffff) as u16)
}


/// Emulate the Minecraft cosinus lookup table.
pub fn mc_cos(x: f32) -> f32 {
    mc_sin_table(((x * 10430.38 + 16384.0) as i32 & 0xffff) as u16)
}
