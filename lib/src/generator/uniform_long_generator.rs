use super::{Generator, NumberGenerator};
use rand::prelude::*;

pub struct UniformLongGenerator {
    lower_bound: u64,
    upper_bound: u64,
}

impl UniformLongGenerator {
    pub fn new(lower_bound: u64, upper_bound: u64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }
}

impl Generator<u64> for UniformLongGenerator {
    fn next_value(&self, rng: &mut SmallRng) -> u64 {
        rng.gen_range(self.lower_bound..=self.upper_bound)
    }
}

impl NumberGenerator<u64> for UniformLongGenerator {
    fn mean(&self) -> u64 {
        (self.lower_bound + self.upper_bound) / 2
    }
}
