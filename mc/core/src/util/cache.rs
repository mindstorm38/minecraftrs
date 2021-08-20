use std::time::Instant;
use std::ops::{Deref, DerefMut};


pub struct TimedCache<T, const M: u64> {
    value: T,
    last_update: Instant
}

impl<T, const M: u64> TimedCache<T, M> {

    pub fn new(value: T) -> Self {
        Self {
            value,
            last_update: Instant::now()
        }
    }

    pub fn refresh(&mut self) -> &mut Self {
        self.last_update = Instant::now();
        self
    }

    pub fn timedout(&self) -> bool {
        self.last_update.elapsed().as_secs() >= M
    }

}

impl<T, const M: u64> Deref for TimedCache<T, M> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, const M: u64> DerefMut for TimedCache<T, M> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
