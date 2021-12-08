use rand::prelude::SmallRng;

use super::Generator;

pub struct ConstantGenerator<T: ToString + Clone> {
    value: T,
}

impl<T: ToString + Clone> ConstantGenerator<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: ToString + Clone> Generator<T> for ConstantGenerator<T> {
    fn next_value(&self, _rng: &mut SmallRng) -> T {
        self.value.clone()
    }
}
