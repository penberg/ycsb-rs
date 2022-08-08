use super::{Generator, NumberGenerator};
use rand::prelude::*;

pub const ZIPFIAN_CONSTANT: f64 = 0.99;

#[allow(dead_code)]
struct ZipfianParameters {
    alpha: f64,
    zetan: f64,
    eta: f64,
    theta: f64,
    zeta2theta: f64,
}

#[allow(dead_code)]
pub struct ZipfianGenerator {
    items: u64,
    base: u64,
    zipfian_constant: f64,
    zipfian_parameters: ZipfianParameters,
    count_for_zeta: u64,
    allow_item_count_decrease: bool,
}

fn zeta_4(st: u64, n: u64, theta: f64, initial_sum: f64) -> f64 {
    let mut sum = initial_sum;
    for i in st..n {
        sum += 1.0 / (i as f64 + 1.0).powf(theta);
    }
    sum
}

fn zeta_2(n: u64, theta: f64) -> f64 {
    zeta_4(0, n, theta, 0.0)
}

impl ZipfianGenerator {
    pub fn from_items(items: u64) -> Self {
        Self::from_range(0, items - 1)
    }

    pub fn from_range(min: u64, max: u64) -> Self {
        Self::from_range_const(min, max, ZIPFIAN_CONSTANT)
    }

    pub fn from_range_const(min: u64, max: u64, zipfian_constant: f64) -> Self {
        Self::new(
            min,
            max,
            zipfian_constant,
            zeta_2(max - min + 1, zipfian_constant),
        )
    }

    pub fn new(min: u64, max: u64, zipfian_constant: f64, zetan: f64) -> Self {
        let theta = zipfian_constant;
        let zeta2theta = zeta_2(2, theta);
        let items = max - min + 1;
        let zipfian_parameters = ZipfianParameters {
            alpha: 1.0 / (1.0 - theta),
            zetan,
            eta: (1.0 - (2.0 / items as f64).powf(1.0 - theta)) / (1.0 - zeta2theta / zetan),
            theta,
            zeta2theta,
        };
        Self {
            items,
            base: min,
            zipfian_constant,
            zipfian_parameters,
            count_for_zeta: items,
            allow_item_count_decrease: false,
        }
    }

    fn next_long(&self, item_count: u64, rng: &mut SmallRng) -> u64 {
        if item_count != self.count_for_zeta {
            /*
            if item_count > self.count_for_zeta {
                warn!("incrementally recomputing Zipfian distribtion (increase)");
                self.zipfian_parameters.zetan = zeta_4(
                    self.count_for_zeta,
                    item_count,
                    self.zipfian_parameters.theta,
                    self.zipfian_parameters.zetan,
                );
            }
            if item_count < self.count_for_zeta && self.allow_item_count_decrease {
                warn!("incrementally recomputing Zipfian distribtion (decrease). This is slow and should be avoided.");
                self.zipfian_parameters.zetan = zeta_2(item_count, self.zipfian_parameters.theta);
            }

            self.count_for_zeta = item_count;
            self.zipfian_parameters.eta = (1.0
                - (2.0 / self.items as f64).powf(1.0 - self.zipfian_parameters.theta))
                / (1.0 - self.zipfian_parameters.zeta2theta / self.zipfian_parameters.zetan);
            */
            todo!("change item count after creating zipfian is not yet supported");
        }

        let u = rng.gen::<f64>();
        let uz = u * self.zipfian_parameters.zetan;

        if uz < 1.0 {
            return self.base;
        }

        if uz < 1.0 + (0.5_f64).powf(self.zipfian_parameters.theta) {
            return self.base + 1;
        }

        self.base
            + (item_count as f64
                * (self.zipfian_parameters.eta * u - self.zipfian_parameters.eta + 1.0)
                    .powf(self.zipfian_parameters.alpha)) as u64
    }
}

impl Generator<u64> for ZipfianGenerator {
    fn next_value(&self, rng: &mut SmallRng) -> u64 {
        self.next_long(self.items, rng)
    }
}

impl NumberGenerator<u64> for ZipfianGenerator {
    fn mean(&self) -> u64 {
        todo!("implement ZipfianGenerator::mean")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_and_max_parameter() {
        let min = 5;
        let max = 10;
        let zipfian = ZipfianGenerator::from_range(min, max);
        let mut result = std::collections::HashMap::new();
        let mut rng = SmallRng::from_entropy();
        for _i in 0..100000 {
            let val = zipfian.next_value(&mut rng);
            assert!(val >= min);
            assert!(val <= max);
            result.entry(val).and_modify(|x| *x += 1).or_insert(1);
        }
        println!("{:?}", result);
    }
}
