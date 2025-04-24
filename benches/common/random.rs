use std::ops::RangeInclusive;

use rand::prelude::*;

pub struct RandomF64Iterator {
    factor: f64,
    start: f64,
    end: f64,
    rng: StdRng,
}

impl RandomF64Iterator {
    pub fn new(seed: u64, precision: usize, range: RangeInclusive<f64>) -> Self {
        RandomF64Iterator {
            factor: 10_f64.powi(precision as i32),
            start: *range.start(),
            end: *range.end(),
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Iterator for RandomF64Iterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.rng.random_range(self.start..=self.end);
        Some((value * self.factor).round() / self.factor)
    }
}

pub struct RandomF64StringIterator {
    inner: RandomF64Iterator,
}

impl RandomF64StringIterator {
    pub fn new(seed: u64, precision: usize, range: RangeInclusive<f64>) -> Self {
        RandomF64StringIterator {
            inner: RandomF64Iterator::new(seed, precision, range),
        }
    }
}

impl Iterator for RandomF64StringIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| format!("{}", f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_f64_iterator() {
        let mut iterator = RandomF64Iterator::new(12345, 3, -100000.0..=100000.0);
        assert_eq!(iterator.next(), Some(-38813.542));
        assert_eq!(iterator.next(), Some(76623.125));
        assert_eq!(iterator.next(), Some(9318.383));

        let mut iterator = RandomF64Iterator::new(12345, 5, -100000.0..=100000.0);
        assert_eq!(iterator.next(), Some(-38813.54153));
        assert_eq!(iterator.next(), Some(76623.12532));
        assert_eq!(iterator.next(), Some(9318.38349));
    }

    #[test]
    fn test_random_f64_iterator_as_str() {
        let mut iterator =
            RandomF64Iterator::new(12345, 3, -100000.0..=100000.0).map(|f| format!("{}", f));
        assert_eq!(iterator.next(), Some("-38813.542".to_string()));
        assert_eq!(iterator.next(), Some("76623.125".to_string()));
        assert_eq!(iterator.next(), Some("9318.383".to_string()));

        let mut iterator =
            RandomF64Iterator::new(12345, 5, -100000.0..=100000.0).map(|f| format!("{}", f));
        assert_eq!(iterator.next(), Some("-38813.54153".to_string()));
        assert_eq!(iterator.next(), Some("76623.12532".to_string()));
        assert_eq!(iterator.next(), Some("9318.38349".to_string()));
    }

    #[test]
    fn test_random_f64_string_iterator() {
        let mut iterator = RandomF64StringIterator::new(12345, 3, -100000.0..=100000.0);
        assert_eq!(iterator.next(), Some("-38813.542".to_string()));
        assert_eq!(iterator.next(), Some("76623.125".to_string()));
        assert_eq!(iterator.next(), Some("9318.383".to_string()));

        let mut iterator = RandomF64StringIterator::new(12345, 5, -100000.0..=100000.0);
        assert_eq!(iterator.next(), Some("-38813.54153".to_string()));
        assert_eq!(iterator.next(), Some("76623.12532".to_string()));
        assert_eq!(iterator.next(), Some("9318.38349".to_string()));
    }
}
