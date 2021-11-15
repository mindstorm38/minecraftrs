use mc_core::rand::JavaRandom;

use crate::view::LevelView;
use super::Feature;


/// A trait to implement to types that can be interpreted as a count for the `RepeatedFeature`.
pub trait RepeatCount {
    fn get_count(&self, rand: &mut JavaRandom) -> u16;
}

impl RepeatCount for u16 {
    fn get_count(&self, _rand: &mut JavaRandom) -> u16 {
        *self
    }
}


/// A feature that repeat a give number of time the given feature.
pub struct RepeatedFeature<F: Feature, C: RepeatCount> {
    feature: F,
    count: C,
}

impl<F: Feature, C: RepeatCount> RepeatedFeature<F, C> {
    pub fn new(feature: F, count: C) -> Self {
        Self {
            feature,
            count,
        }
    }
}

impl<F: Feature, C: RepeatCount> Feature for RepeatedFeature<F, C> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        let count = self.count.get_count(rand);
        for _ in 0..count {
            self.feature.generate(level, rand, x, y, z);
        }
        true  // TODO
    }
}


pub struct OptionalFeature<F: Feature, E: Feature> {
    if_feature: F,
    else_feature: E,
    bound: i32,
}

impl<F: Feature, E: Feature> OptionalFeature<F, E> {
    pub fn new(if_feature: F, else_feature: E, bound: u16) -> Self {
        Self {
            if_feature,
            else_feature,
            bound: bound as i32,
        }
    }
}

impl<F: Feature, E: Feature> Feature for OptionalFeature<F, E> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        if rand.next_int_bounded(self.bound) == 0 {
            self.if_feature.generate(level, rand, x, y, z)
        } else {
            self.else_feature.generate(level, rand, x, y, z)
        }
    }
}


pub struct ChainFeature<A: Feature, B: Feature> {
    a_feature: A,
    b_feature: B,
}

impl<A: Feature, B: Feature> ChainFeature<A, B> {
    pub fn new(a_feature: A, b_feature: B) -> Self {
        Self { a_feature, b_feature }
    }
}

impl<A: Feature, B: Feature> Feature for ChainFeature<A, B> {
    fn generate(&self, level: &mut dyn LevelView, rand: &mut JavaRandom, x: i32, y: i32, z: i32) -> bool {
        let a_success = self.a_feature.generate(level, rand, x, y, z);
        let b_success = self.b_feature.generate(level, rand, x, y, z);
        a_success && b_success
    }
}
