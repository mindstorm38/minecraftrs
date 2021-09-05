use std::time::{Instant, Duration};
use std::ops::{Deref, DerefMut};


pub struct TimedCache<T> {
    value: T,
    last_update: Instant,
    lifetime: Duration
}

impl<T> TimedCache<T> {

    pub fn new(value: T, lifetime: Duration) -> Self {
        Self {
            value,
            last_update: Instant::now(),
            lifetime
        }
    }

    pub fn cache_update(&mut self) -> &mut Self {
        self.last_update = Instant::now();
        self
    }

    pub fn is_cache_timed_out(&self) -> bool {
        self.last_update.elapsed() >= self.lifetime
    }

}

impl<T> Deref for TimedCache<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for TimedCache<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
