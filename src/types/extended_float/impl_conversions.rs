use std::convert::From;
use std::fmt;

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;
use crate::utils::check_invalid_float;

/// Error type for conversion failures when creating ExtendedFloat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionError {
    /// Value is NaN
    NaN,
    /// Value is infinite
    Infinite,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::NaN => write!(f, "Cannot create ExtendedFloat from NaN"),
            ConversionError::Infinite => {
                write!(f, "Cannot create ExtendedFloat from infinite value")
            }
        }
    }
}

impl std::error::Error for ConversionError {}

impl<T: DisplayableFloat> From<T> for ExtendedFloat<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: DisplayableFloat> ExtendedFloat<T> {
    /// Try to create ExtendedFloat safely without panicking
    ///
    /// Unlike the `new()` constructor, this method returns a Result with error details
    /// instead of panicking when given NaN or infinity values.
    #[inline]
    pub fn try_from_value(value: T) -> Result<Self, ConversionError> {
        if let Some((is_nan, _)) = check_invalid_float(value) {
            if is_nan {
                Err(ConversionError::NaN)
            } else {
                Err(ConversionError::Infinite)
            }
        } else {
            Ok(Self::new_unchecked(value))
        }
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
    fn test_try_from_value() {
        assert!(ExtendedFloat::try_from_value(1.0).is_ok());
        assert!(ExtendedFloat::try_from_value(-100.5).is_ok());
        assert!(ExtendedFloat::try_from_value(0.0).is_ok());

        assert_eq!(
            ExtendedFloat::try_from_value(f64::NAN).unwrap_err(),
            ConversionError::NaN
        );
        assert_eq!(
            ExtendedFloat::try_from_value(f64::INFINITY).unwrap_err(),
            ConversionError::Infinite
        );
        assert_eq!(
            ExtendedFloat::try_from_value(f64::NEG_INFINITY).unwrap_err(),
            ConversionError::Infinite
        );
    }

    #[test]
    #[should_panic(expected = "ExtendedFloat doesn't support NaN values")]
    fn test_from_panics_on_nan() {
        ExtendedFloat::from(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "ExtendedFloat doesn't support infinite values")]
    fn test_from_panics_on_infinity() {
        ExtendedFloat::from(f64::INFINITY);
    }

    #[test]
    fn test_from_valid_values() {
        let value = 123.456;
        let extended = ExtendedFloat::from(value);
        assert_eq!(extended.downgrade(), value);

        let value = -123.456;
        let extended = ExtendedFloat::from(value);
        assert_eq!(extended.downgrade(), value);

        let extended = ExtendedFloat::from(PI);
        assert_eq!(extended.downgrade(), PI);

        let extended = ExtendedFloat::from(0.0);
        assert_eq!(extended.downgrade(), 0.0);
    }

    #[test]
    fn test_into_f64() {
        let extended = ExtendedFloat::new(123.456);
        let value: f64 = extended.into();
        assert_eq!(value, 123.456);

        let extended = ExtendedFloat::new(-123.456);
        let value: f64 = extended.into();
        assert_eq!(value, -123.456);
    }
}
