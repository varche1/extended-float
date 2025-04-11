use std::convert::From;

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> From<T> for ExtendedFloat<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: DisplayableFloat> From<ExtendedFloat<T>> for String {
    fn from(value: ExtendedFloat<T>) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_from_trait() {
        // Test converting from an f64
        let value = 123.456;
        let extended = ExtendedFloat::from(value);
        assert_eq!(extended.0, value);

        // Test with negative values
        let value = -123.456;
        let extended = ExtendedFloat::from(value);
        assert_eq!(extended.0, value);

        // Test with special values
        let extended = ExtendedFloat::from(PI);
        assert_eq!(extended.0, PI);

        let extended = ExtendedFloat::from(0.0);
        assert_eq!(extended.0, 0.0);

        let extended = ExtendedFloat::from(f64::INFINITY);
        assert!(extended.0.is_infinite());
        assert!(extended.0.is_sign_positive());

        let extended = ExtendedFloat::from(f64::NEG_INFINITY);
        assert!(extended.0.is_infinite());
        assert!(extended.0.is_sign_negative());

        let extended = ExtendedFloat::from(f64::NAN);
        assert!(extended.0.is_nan());
    }
}
