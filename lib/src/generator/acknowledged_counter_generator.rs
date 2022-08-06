use super::{CounterGenerator, Generator};
use rand::prelude::*;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Mutex,
};

const WINDOW_SIZE: u64 = 1 << 20;
const WINDOW_MASK: u64 = WINDOW_SIZE - 1;

pub struct AcknowledgedCounterGenerator {
    counter: CounterGenerator,
    window: Vec<AtomicBool>,
    limit: AtomicU64,
    core: Mutex<()>,
}

impl AcknowledgedCounterGenerator {
    pub fn new(count_start: u64) -> Self {
        let counter = CounterGenerator::new(count_start);
        let mut window = Vec::with_capacity(WINDOW_SIZE as usize);
        for _i in 0..WINDOW_SIZE {
            window.push(AtomicBool::new(false));
        }
        Self {
            counter,
            window,
            limit: AtomicU64::new(count_start - 1),
            core: Mutex::new(()),
        }
    }

    pub fn acknowledge(&self, value: u64) {
        let current_slot = value & WINDOW_MASK;
        let slot = &self.window[current_slot as usize];
        if slot.swap(true, Ordering::SeqCst) {
            panic!("too many unacknowledged requests");
        }
        if let Ok(_lock) = self.core.try_lock() {
            let limit = self.limit.load(Ordering::SeqCst);
            let before_first_slot = limit & WINDOW_MASK;
            let mut index = limit + 1;
            let new_index = loop {
                if index != before_first_slot {
                    let slot = (index & WINDOW_MASK) as usize;
                    if !self.window[slot].load(Ordering::SeqCst) {
                        break index;
                    }
                    self.window[slot].store(false, Ordering::SeqCst);
                } else {
                    break index;
                }
                index += 1;
            };
            self.limit.store(new_index - 1, Ordering::SeqCst);
        }
    }

    pub fn last_value(&self) -> u64 {
        self.limit.load(Ordering::SeqCst)
    }
}

impl Generator<u64> for AcknowledgedCounterGenerator {
    fn next_value(&self, rng: &mut SmallRng) -> u64 {
        self.counter.next_value(rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let generator = AcknowledgedCounterGenerator::new(1);
        let mut rng = SmallRng::from_entropy();
        assert_eq!(generator.next_value(&mut rng), 1);
        assert_eq!(generator.last_value(), 0);
        assert_eq!(generator.next_value(&mut rng), 2);
        assert_eq!(generator.last_value(), 0);
        generator.acknowledge(1);
        assert_eq!(generator.last_value(), 1);
        generator.acknowledge(2);
        assert_eq!(generator.last_value(), 2);
        generator.acknowledge(1);
        assert_eq!(generator.last_value(), 2);
    }
}
