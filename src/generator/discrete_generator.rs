use super::Generator;
use rand::prelude::*;

pub struct WeightPair<T: Clone + Send> {
    weight: f64,
    value: T,
}

impl<T: Clone + Send> WeightPair<T> {
    pub fn new(weight: f64, value: impl Into<T>) -> Self {
        Self {
            weight,
            value: value.into(),
        }
    }
}

pub struct DiscreteGenerator<T: Clone + Send> {
    values: Vec<WeightPair<T>>,
    sum: f64,
}

impl<T: ToString + Clone + Send> DiscreteGenerator<T> {
    pub fn new(values: Vec<WeightPair<T>>) -> Self {
        let mut sum = 0.0;
        for WeightPair { weight, .. } in &values {
            sum += *weight;
        }
        Self { values, sum }
    }
}

impl<T: ToString + Clone + Send> Generator<T> for DiscreteGenerator<T> {
    fn next_value(&self, rng: &mut SmallRng) -> T {
        let mut val = rng.gen::<f64>();
        for WeightPair { weight, value } in &self.values {
            let pw = *weight / self.sum;
            if val < pw {
                return value.clone();
            }
            val -= pw;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrete_generator() {
        let weight_pairs = vec![WeightPair::new(0.3, "test"), WeightPair::new(0.7, "b")];
        let generator = DiscreteGenerator::<String>::new(weight_pairs);
        let mut result = std::collections::HashMap::new();
        let mut rng = SmallRng::from_entropy();
        for _i in 0..10000 {
            let val = generator.next_value(&mut rng);
            result.entry(val).and_modify(|x| *x += 1).or_insert(1);
        }
        println!("{:?}", result);
    }
}
