use std::convert::From;
use std::fmt;
use std::str::FromStr;

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

#[derive(Debug, PartialEq, Eq)]
pub enum FromStrError {
    Invalid,
}

impl<T: DisplayableFloat + FromStr + fast_float::FastFloat> FromStr for ExtendedFloat<T> {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: T = fast_float::parse(s).map_err(|_| FromStrError::Invalid)?;
        ExtendedFloat::try_from_value(v).map_err(|_| FromStrError::Invalid)
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
            Ok(unsafe { Self::new_unchecked(value) })
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

    use pretty_assertions::assert_eq;

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
        let _ = ExtendedFloat::from(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "ExtendedFloat doesn't support infinite values")]
    fn test_from_panics_on_infinity() {
        let _ = ExtendedFloat::from(f64::INFINITY);
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

    #[test]
    fn test_from_str_valid() {
        let ef: ExtendedFloat<f64> = "42.42".parse().unwrap();
        assert_eq!(ef.downgrade(), 42.42);

        let ef: ExtendedFloat<f64> = "42".parse().unwrap();
        assert_eq!(ef.downgrade(), 42.0);

        // Scientific notation
        let ef: ExtendedFloat<f64> = "1.5e3".parse().unwrap();
        assert_eq!(ef.downgrade(), 1500.0);

        let ef: ExtendedFloat<f64> = "1.5E-3".parse().unwrap();
        assert_eq!(ef.downgrade(), 0.0015);

        // Negative numbers
        let ef: ExtendedFloat<f64> = "-123.456".parse().unwrap();
        assert_eq!(ef.downgrade(), -123.456);

        // Edge cases
        let ef: ExtendedFloat<f64> = "0.0".parse().unwrap();
        assert_eq!(ef.downgrade(), 0.0);

        let ef: ExtendedFloat<f64> = "0".parse().unwrap();
        assert_eq!(ef.downgrade(), 0.0);

        let ef: ExtendedFloat<f64> = "9007199254740992".parse().unwrap(); // 2^53
        assert_eq!(ef.downgrade(), 9007199254740992.0);
    }

    #[test]
    fn test_from_str_invalid() {
        let err = "not_a_number".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "-".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "+".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "1/1".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        // Special floating point values (not supported by ExtendedFloat)
        let err = "NaN".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "Infinity".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "-Infinity".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        // Malformed numbers
        let err = "1.2.3".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "1e".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "e10".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "123abc".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        let err = "0x123".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);

        // Whitespace handling must be somewhere over parsing
        let err = "  42.5  ".parse::<ExtendedFloat<f64>>().unwrap_err();
        assert_eq!(err, FromStrError::Invalid);
    }
}
