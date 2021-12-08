use super::Generator;
use rand::prelude::*;
use std::sync::atomic::AtomicU64;

pub struct CounterGenerator {
    counter: AtomicU64,
}

impl CounterGenerator {
    pub fn new(count_start: u64) -> Self {
        Self {
            counter: AtomicU64::new(count_start),
        }
    }
}

impl Generator<u64> for CounterGenerator {
    fn next_value(&self, _rng: &mut SmallRng) -> u64 {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
