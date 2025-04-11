use crate::constants::f64::{
    DECIMAL_PRECISION, DECIMAL_PRECISION_DIGITS, EPSILON, MAX_EXPONENT, MIN_EXPONENT,
};
use crate::tables::lookup::{EXTRA_DIGITS_TABLE, PRECISION_TABLE};
use crate::traits::Float;

impl Float for f64 {
    fn epsilon() -> f64 {
        EPSILON
    }

    fn exponent(&self) -> i16 {
        // IEEE 754 double-precision format:
        // - 1 bit sign
        // - 11 bits exponent (with bias of 1023)
        // - 52 bits fraction (mantissa)
        let bits = self.to_bits();
        let exponent_bits = (bits >> 52) & 0x7FF; // Extract the 11 exponent bits
        let bias = 1023; // Exponent bias for f64
        (exponent_bits as i32 - bias) as i16 // Remove bias to get the actual exponent
    }

    fn decimal_precision() -> f64 {
        DECIMAL_PRECISION
    }

    fn decimal_precision_digits() -> u16 {
        DECIMAL_PRECISION_DIGITS
    }

    fn extra_digits(&self) -> u16 {
        let exponent = self.exponent();

        // Handle special cases or out-of-range exponents
        if !(MIN_EXPONENT..=MAX_EXPONENT).contains(&exponent) {
            return 0; // Fallback for any out-of-range values (NaN, Infinity, subnormals)
        }

        // Use lookup table for fast conversion from binary exponent to extra decimal digits
        // This avoids runtime log10(2) multiplication and ceiling operations
        let index = (exponent - MIN_EXPONENT) as usize;
        EXTRA_DIGITS_TABLE[index]
    }

    fn precision(&self) -> usize {
        let exponent = self.exponent();

        // Handle special cases or check if exponent is within our lookup table range
        if !(MIN_EXPONENT..=MAX_EXPONENT).contains(&exponent) {
            // For out-of-range values, calculate precision dynamically
            let extra_digits = self.extra_digits();
            if extra_digits > Self::decimal_precision_digits() {
                return 0; // No precision left if extra_digits exceeds our total precision
            } else {
                return (Self::decimal_precision_digits() - extra_digits) as usize;
            }
        }

        // Use the lookup table for fast precision retrieval
        // This approach is significantly faster than calculating at runtime
        // and provides consistent precision handling across all operations
        let index = (exponent - MIN_EXPONENT) as usize;
        PRECISION_TABLE[index]
    }
}
