mod acknowledged_counter_generator;
mod constant_generator;
mod counter_generator;
mod discrete_generator;
mod uniform_long_generator;
mod zipfian_generator;

pub use acknowledged_counter_generator::AcknowledgedCounterGenerator;
pub use constant_generator::ConstantGenerator;
pub use counter_generator::CounterGenerator;
pub use discrete_generator::{DiscreteGenerator, WeightPair};
use rand::prelude::SmallRng;
pub use uniform_long_generator::UniformLongGenerator;
pub use zipfian_generator::ZipfianGenerator;

use std::string::ToString;

pub trait Generator<T: ToString + Clone + Send> {
    fn next_value(&self, rng: &mut SmallRng) -> T;
}

pub trait NumberGenerator<T: ToString + Clone + Send>: Generator<T> {
    fn mean(&self) -> T;
}

pub struct GeneratorImpl<T: ToString + Clone + Send, G: Generator<T>> {
    last_value: Option<T>,
    generator: G,
}

impl<T, G> GeneratorImpl<T, G>
where
    G: Generator<T>,
    T: ToString + Clone + Send,
{
    pub fn new(generator: G) -> Self {
        Self {
            generator,
            last_value: None,
        }
    }

    pub fn next_value(&mut self, rng: &mut SmallRng) -> T {
        let v = self.generator.next_value(rng);
        self.last_value = Some(v.clone());
        v
    }

    pub fn last_value(&self) -> T {
        self.last_value.clone().unwrap()
    }

    pub fn next_string(&mut self, rng: &mut SmallRng) -> String {
        self.next_value(rng).to_string()
    }

    pub fn last_string(&self) -> String {
        self.last_value().to_string()
    }
}
