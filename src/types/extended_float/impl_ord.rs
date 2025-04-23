use std::cmp::{Ord, Ordering, PartialOrd};

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

// TODO: bench

impl<T: DisplayableFloat> Ord for ExtendedFloat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        if self.downgrade() > other.downgrade() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<T: DisplayableFloat> PartialOrd for ExtendedFloat<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::constants::f64::EPSILON;

    #[test]
    fn test_ordering_basics() {
        assert!(ExtendedFloat::new(2.0) > ExtendedFloat::new(1.0));
        assert!(ExtendedFloat::new(1.0) < ExtendedFloat::new(2.0));
        assert!(ExtendedFloat::new(1.0) <= ExtendedFloat::new(1.0));
        assert!(ExtendedFloat::new(1.0) >= ExtendedFloat::new(1.0));

        assert!(ExtendedFloat::new(1.0) == ExtendedFloat::new(1.0 + EPSILON * 0.5));
        assert!(ExtendedFloat::new(1.0) < ExtendedFloat::new(1.0 + EPSILON * 50.0));
        assert!(ExtendedFloat::new(1.0 + EPSILON * 50.0) > ExtendedFloat::new(1.0));
    }

    #[test]
    fn test_sorting() {
        let mut values = [
            ExtendedFloat::new(5.0),
            ExtendedFloat::new(2.0),
            ExtendedFloat::new(3.0),
            ExtendedFloat::new(1.0),
            ExtendedFloat::new(4.0),
        ];

        values.sort();

        assert_eq!(values[0], ExtendedFloat::new(1.0));
        assert_eq!(values[1], ExtendedFloat::new(2.0));
        assert_eq!(values[2], ExtendedFloat::new(3.0));
        assert_eq!(values[3], ExtendedFloat::new(4.0));
        assert_eq!(values[4], ExtendedFloat::new(5.0));
    }
}
