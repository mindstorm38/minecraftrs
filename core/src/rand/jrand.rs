use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{UNIX_EPOCH, SystemTime};
use std::num::Wrapping;


const MULTIPLIER: Wrapping<i64> = Wrapping(0x5DEECE66D);
const ADDEND: Wrapping<i64> = Wrapping(0xB);
const MASK: Wrapping<i64> = Wrapping((1 << 48) - 1);

const FLOAT_DIV: f32 = (1u32 << 24) as f32;
const DOUBLE_DIV: f64 = (1u64 << 53) as f64;


#[inline]
pub fn initial_scramble(seed: i64) -> Wrapping<i64> {
    (Wrapping(seed) ^ MULTIPLIER) & MASK
}


/// Generate a new seed in the same way as `java.util.Random` (same constants).
pub fn gen_seed() -> i64 {
    static mut SEED: AtomicI64 = AtomicI64::new(8682522807148012);
    unsafe {
        loop {
            let current = SEED.load(Ordering::Relaxed);
            let next = current.wrapping_mul(181783497276652981);
            if SEED.compare_exchange(current, next, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                // This is a bit different from Java implementation because the nano time
                // as an integer value is not available in Rust, even with Instant.
                // So we're using duration since unix epoch of the system time, maybe not
                // as safe as the Java implementation.
                return match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(d) => next ^ (d.as_nanos() as i64),
                    Err(_) => next
                };
            }
        }
    }
}


pub struct JavaRandom {
    seed: Wrapping<i64>
}

impl JavaRandom {

    #[inline]
    pub fn new(seed: i64) -> JavaRandom {
        JavaRandom { seed: initial_scramble(seed) }
    }

    #[inline]
    pub fn new_seeded() -> JavaRandom {
        Self::new(gen_seed())
    }

    #[inline]
    pub fn new_blank() -> JavaRandom {
        JavaRandom { seed: Wrapping(0) }
    }

    #[inline]
    pub fn set_seed(&mut self, seed: i64) {
        self.seed = initial_scramble(seed);
    }

    #[inline]
    pub fn get_seed(&self) -> i64 {
        self.seed.0
    }

    pub fn next_blank(&mut self) {
        self.seed = (self.seed * MULTIPLIER + ADDEND) & MASK;
    }

    #[inline]
    fn next(&mut self, bits: u8) -> i32 {
        self.next_blank();
        (self.seed.0 as u64 >> (48 - bits)) as i32
    }

    #[inline]
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_int_bounded(&mut self, bound: i32) -> i32 {

        if (bound & -bound) == bound {
            (((bound as i64).wrapping_mul(self.next(31) as i64)) >> 31) as i32
        } else {

            let mut bits;
            let mut val;

            loop {
                bits = self.next(31);
                val = bits.rem_euclid(bound);
                if bits - val + (bound - 1) >= 0 {
                    break;
                }
            }

            val

        }

    }

    pub fn next_long(&mut self) -> i64 {
        ((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
    }

    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / FLOAT_DIV
    }

    pub fn next_double(&mut self) -> f64 {
        let high = (self.next(26) as i64) << 27;
        let low = self.next(27) as i64;
        (high.wrapping_add(low) as f64) / DOUBLE_DIV
    }

}
