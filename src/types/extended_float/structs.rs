use std::fmt::Write;

use crate::traits::{DisplayableFloat, Float};

#[derive(Debug, Clone, Copy)]
pub struct ExtendedFloat<T: DisplayableFloat>(pub T);

// TODO: make a forbidden of NaN, Inf, etc...
impl<T: DisplayableFloat> ExtendedFloat<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Formats a floating-point number as a string with precision that accounts for floating-point errors.
    ///
    /// This method intelligently formats floating-point numbers by:
    /// 1. Using dynamically calculated precision based on the number's binary exponent
    /// 2. Treating values smaller than epsilon as zero
    /// 3. Preserving very large values (> decimal_precision) without modification
    /// 4. Removing trailing zeros and decimal point when unnecessary
    ///
    /// Note: Values larger than 1/epsilon are formatted as-is since errors occur in the
    /// integer part of the number, and rounding the integer part shouldn't be done in
    /// trading/financial contexts.
    pub fn format(&self) -> String {
        let value = self.0;
        let precision = self.0.precision();

        // Handle special case of zero
        if value.abs() <= <T as Float>::epsilon() {
            return "0".to_string();
        }

        // if value is too big, just return it as is
        if value.abs() > T::decimal_precision() {
            return value.to_string();
        }

        let mut formatted = String::with_capacity(32);
        write!(formatted, "{:.*}", precision, value).unwrap();

        // Remove trailing zeros and decimal point if needed
        if let Some(dot_index) = formatted.find('.') {
            let trimmed_len = formatted[dot_index..]
                .trim_end_matches('0')
                .trim_end_matches('.')
                .len();
            formatted.truncate(dot_index + trimmed_len);
        }

        formatted
    }
}

#[cfg(test)]
#[allow(clippy::excessive_precision)]
mod tests {
    use super::*;
    use crate::constants::f64::{
        DECIMAL_PRECISION as DECIMAL_PRECISION_F64, EPSILON as EPSILON_F64,
    };

    mod format_tests {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn test_zero_formatting() {
            // Test values near zero are formatted as "0"
            assert_eq!(ExtendedFloat(EPSILON_F64).to_string(), "0");
            assert_eq!(ExtendedFloat(-EPSILON_F64).to_string(), "0");
            assert_eq!(ExtendedFloat(0.0000000000000001).to_string(), "0");
        }

        #[test]
        fn test_special_values() {
            // Test NaN and infinity formatting
            assert_eq!(ExtendedFloat(f64::NAN).to_string(), "NaN");
            assert_eq!(ExtendedFloat(f64::INFINITY).to_string(), "inf");
            assert_eq!(ExtendedFloat(f64::NEG_INFINITY).to_string(), "-inf");
        }

        #[test]
        fn test_trailing_zeros_removal() {
            // Test trailing zeros and decimal point are removed correctly
            assert_eq!(ExtendedFloat(123.4560).to_string(), "123.456");
            assert_eq!(ExtendedFloat(123.0000).to_string(), "123");
            assert_eq!(ExtendedFloat(0.00100).to_string(), "0.001");
            assert_eq!(ExtendedFloat(1234.50).to_string(), "1234.5");
        }

        #[test]
        fn test_decimal_precision() {
            // Test regular decimals with different precision
            assert_eq!(ExtendedFloat(0.1).to_string(), "0.1");
            assert_eq!(ExtendedFloat(0.01).to_string(), "0.01");
            assert_eq!(ExtendedFloat(0.001).to_string(), "0.001");
            assert_eq!(ExtendedFloat(0.0001).to_string(), "0.0001");
            assert_eq!(ExtendedFloat(0.12345678).to_string(), "0.12345678");
            assert_eq!(ExtendedFloat(1234.56).to_string(), "1234.56");
        }

        #[test]
        fn test_rounding_behavior() {
            // Test rounding behavior at precision boundaries
            assert_eq!(ExtendedFloat(0.5000000000000001).to_string(), "0.5");
            assert_eq!(ExtendedFloat(0.4999999999999999).to_string(), "0.5");
            assert_eq!(ExtendedFloat(1.5000000000000001).to_string(), "1.5");
        }

        #[test]
        fn test_precision_boundary() {
            // Test values at the precision boundary (14-15 digits)
            assert_eq!(
                ExtendedFloat(4.00000000000001).to_string(),
                "4.00000000000001"
            );
            assert_eq!(ExtendedFloat(4.000000000000001).to_string(), "4");
            assert_eq!(
                ExtendedFloat(-4.00000000000001).to_string(),
                "-4.00000000000001"
            );
            assert_eq!(ExtendedFloat(-4.000000000000001).to_string(), "-4");
        }

        #[test]
        fn test_constants() {
            // Test mathematical constants
            assert_eq!(
                ExtendedFloat(std::f64::consts::PI).to_string(),
                "3.14159265358979"
            );
            assert_eq!(
                ExtendedFloat(std::f64::consts::E).to_string(),
                "2.71828182845905"
            );
        }

        #[test]
        fn test_values_near_one() {
            // Test values near 1.0 with different precision
            assert_eq!(
                ExtendedFloat(0.999999999999999).to_string(),
                "0.999999999999999"
            );
            assert_eq!(ExtendedFloat(0.9999999999999999).to_string(), "1");
            assert_eq!(
                ExtendedFloat(-0.999999999999999).to_string(),
                "-0.999999999999999"
            );
            assert_eq!(ExtendedFloat(-0.9999999999999999).to_string(), "-1");
            assert_eq!(
                ExtendedFloat(0.99999999999999).to_string(),
                "0.99999999999999"
            );
            assert_eq!(
                ExtendedFloat(0.999999999999995).to_string(),
                "0.999999999999995"
            );
            assert_eq!(
                ExtendedFloat(0.9999999999999995).to_string(),
                "0.999999999999999"
            );
            assert_eq!(
                ExtendedFloat(0.999999999999991).to_string(),
                "0.999999999999991"
            );
            assert_eq!(
                ExtendedFloat(0.9999999999999991).to_string(),
                "0.999999999999999"
            );
        }

        #[test]
        fn test_border_cases() {
            // Border cases, should be formatted as is
            assert_eq!(ExtendedFloat(f64::MAX).to_string(), f64::MAX.to_string());
            assert_eq!(ExtendedFloat(f64::MIN).to_string(), f64::MIN.to_string());

            // Sub-border cases which can lead to inf
            let max_sub_border_plus = f64::MAX / DECIMAL_PRECISION_F64 + 1.0;
            let min_sub_border_minus = f64::MIN / DECIMAL_PRECISION_F64 - 1.0;
            assert_eq!(
                ExtendedFloat(max_sub_border_plus).to_string(),
                max_sub_border_plus.to_string()
            );
            assert_eq!(
                ExtendedFloat(min_sub_border_minus).to_string(),
                min_sub_border_minus.to_string()
            );

            // Sub-border cases which almost can lead to inf
            let max_sub_border_minus = f64::MAX / DECIMAL_PRECISION_F64 - 1.0;
            let min_sub_border_plus = f64::MIN / DECIMAL_PRECISION_F64 + 1.0;
            assert_eq!(
                ExtendedFloat(max_sub_border_minus).to_string(),
                max_sub_border_minus.to_string()
            );
            assert_eq!(
                ExtendedFloat(min_sub_border_plus).to_string(),
                min_sub_border_plus.to_string()
            );
        }

        #[test]
        fn test_dynamic_precision() {
            // Test dynamically calculated precision cases
            for i in 0..(DECIMAL_PRECISION_F64.log10().abs().ceil() + 5.) as usize {
                let precision = if i >= 15 { 0 } else { i };

                for j in 1..5 {
                    let value = 4.0 + (j as f64) / 10.0_f64.powf(i as f64);
                    let expected = format!("{:.*}", precision, value);
                    let actual = ExtendedFloat(value).to_string();
                    assert_eq!(
                        actual, expected,
                        "Failed for precision {} value {}",
                        i, value
                    );
                }
            }
        }

        #[test]
        fn test_parse_format_equivalence() {
            // Test that parsing the formatted string back to f64 gives equivalent results
            let test_values = vec![
                1.23456789,
                0.0000123456789,
                123456.789,
                -1.23456789,
                0.999999999999999,
                1.0,
                0.1,
                0.01,
                0.001,
            ];

            for value in test_values {
                let formatted = ExtendedFloat(value).to_string();
                let parsed: f64 = formatted.parse().unwrap();

                // Check that difference is within epsilon
                assert!(
                    (value - parsed).abs() <= EPSILON_F64 * 10.0,
                    "Failed equivalence for value: {}, formatted: {}, parsed: {}, diff: {}",
                    value,
                    formatted,
                    parsed,
                    (value - parsed).abs()
                );
            }
        }
    }
}
