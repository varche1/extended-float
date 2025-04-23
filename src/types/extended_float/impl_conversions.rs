use std::convert::From;

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> From<T> for ExtendedFloat<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: DisplayableFloat> From<ExtendedFloat<T>> for String {
    fn from(value: ExtendedFloat<T>) -> Self {
        value.to_string()
    }
}

// TODO: bench
impl<T: DisplayableFloat + Into<f64>> From<ExtendedFloat<T>> for f64 {
    fn from(value: ExtendedFloat<T>) -> Self {
        value.downgrade().into()
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
        assert_eq!(extended.downgrade(), value);

        // Test with negative values
        let value = -123.456;
        let extended = ExtendedFloat::from(value);
        assert_eq!(extended.downgrade(), value);

        // Test with special values
        let extended = ExtendedFloat::from(PI);
        assert_eq!(extended.downgrade(), PI);

        let extended = ExtendedFloat::from(0.0);
        assert_eq!(extended.downgrade(), 0.0);

        let extended = ExtendedFloat::from(f64::INFINITY);
        assert!(extended.downgrade().is_infinite());
        assert!(extended.downgrade().is_sign_positive());

        let extended = ExtendedFloat::from(f64::NEG_INFINITY);
        assert!(extended.downgrade().is_infinite());
        assert!(extended.downgrade().is_sign_negative());

        let extended = ExtendedFloat::from(f64::NAN);
        assert!(extended.downgrade().is_nan());
    }

    #[test]
    fn test_into_f64() {
        // Test converting from ExtendedFloat<f64> to f64
        let extended = ExtendedFloat::new(123.456);
        let value: f64 = extended.into();
        assert_eq!(value, 123.456);

        // Test with negative values
        let extended = ExtendedFloat::new(-123.456);
        let value: f64 = extended.into();
        assert_eq!(value, -123.456);

        // Test with special values
        let extended = ExtendedFloat::new(f64::INFINITY);
        let value: f64 = extended.into();
        assert!(value.is_infinite() && value.is_sign_positive());

        let extended = ExtendedFloat::new(f64::NEG_INFINITY);
        let value: f64 = extended.into();
        assert!(value.is_infinite() && value.is_sign_negative());

        let extended = ExtendedFloat::new(f64::NAN);
        let value: f64 = extended.into();
        assert!(value.is_nan());
    }
}
