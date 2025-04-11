use super::ExtendedFloat;
use crate::traits::{DisplayableFloat, OptionEq};

impl<T: DisplayableFloat> OptionEq for Option<ExtendedFloat<T>> {
    fn equal_option(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::f64::EPSILON as EPSILON_F64;

    #[test]
    fn test_option_equality() {
        let none_f64: Option<ExtendedFloat<f64>> = None;
        assert!(none_f64.equal_option(&none_f64));

        assert!(Some(ExtendedFloat(1.0)).equal_option(&Some(ExtendedFloat(1.0))));

        assert!(
            Some(ExtendedFloat(1.0)).equal_option(&Some(ExtendedFloat(1.0 + EPSILON_F64 * 0.5)))
        );

        assert!(!none_f64.equal_option(&Some(ExtendedFloat(1.0))));
        assert!(!Some(ExtendedFloat(1.0)).equal_option(&none_f64));

        assert!(!Some(ExtendedFloat(1.0)).equal_option(&Some(ExtendedFloat(2.0))));
        assert!(
            !Some(ExtendedFloat(1.0)).equal_option(&Some(ExtendedFloat(1.0 + EPSILON_F64 * 10.0)))
        );
    }
}
