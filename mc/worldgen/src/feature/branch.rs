use mc_core::rand::JavaRandom;

use super::{Feature, LevelView};


/// A feature that repeat a give number of time the given feature.
pub struct RepeatedFeature<F: Feature> {
    feature: F,
    count: u16,
}

impl<F: Feature> RepeatedFeature<F> {
    pub fn new(feature: F, count: u16) -> Self {
        Self {
            feature,
            count,
        }
    }
}

impl<F: Feature> Feature for RepeatedFeature<F> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        for _ in 0..self.count {
            self.feature.generate(level, rand, x, y, z);
        }
        true  // TODO
    }
}


pub struct OptionalFeature<F: Feature> {
    feature: F,
    bound: i32,
}

impl<F: Feature> OptionalFeature<F> {
    pub fn new(feature: F, bound: u16) -> Self {
        Self {
            feature,
            bound: bound as i32,
        }
    }
}

impl<F: Feature> Feature for OptionalFeature<F> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        if rand.next_int_bounded(self.bound) == 0 {
            self.feature.generate(level, rand, x, y, z)
        } else {
            false
        }
    }
}
