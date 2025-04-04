use crate::constants::f64::{DECIMAL_PRECISION_DIGITS, EXPONENT_RANGE, MIN_EXPONENT};

/// Pre-computed lookup table for extra digits needed when representing binary floating-point values in decimal.
///
/// For each possible f64 exponent (-1023 to 1024), this table stores the number of decimal digits
/// that would be consumed by the binary-to-decimal conversion overhead.
///
/// Formula: ceiling(exponent * log10(2)) where log10(2) ≈ 0.301029995663981
pub const EXTRA_DIGITS_TABLE: [u16; EXPONENT_RANGE] = generate_extra_digits_table();

/// Pre-computed lookup table for effective decimal precision available for each exponent.
///
/// For each possible f64 exponent, this stores the effective precision (in decimal digits)
/// available for meaningful representation after accounting for binary-to-decimal conversion.
///
/// Formula: max(0, DECIMAL_PRECISION_DIGITS - extra_digits)
pub const PRECISION_TABLE: [usize; EXPONENT_RANGE] = generate_precision_table();

/// Generates a lookup table of extra decimal digits needed for each binary exponent.
///
/// The binary-to-decimal conversion requires additional decimal digits to represent
/// the same precision. This function pre-computes these values at compile time.
///
/// Technical details:
/// - Uses fixed-point arithmetic for compile-time calculation (const fn restrictions)
/// - Computes ceiling(exponent * log10(2)) for each possible exponent
/// - For negative exponents resulting in negative values, returns 0 (they don't require extra digits)
const fn generate_extra_digits_table() -> [u16; EXPONENT_RANGE] {
    // Convert LOG10_2 to fixed point for compile-time calculations
    // 0.301029995663981 * 1_000_000 = 301030
    const LOG10_2_FIXED: i32 = 301030;
    const FIXED_POINT_SCALE: i32 = 1_000_000;

    let mut table = [0u16; EXPONENT_RANGE];
    let mut i = 0;

    while i < EXPONENT_RANGE {
        // Convert array index to actual exponent
        let exp = (i as i32) + (MIN_EXPONENT as i32);

        // Fixed-point multiplication of exponent * LOG10_2
        let mut value = exp * LOG10_2_FIXED;

        // Apply ceiling function: if there's a fractional part, round up
        if value > 0 && value % FIXED_POINT_SCALE != 0 {
            value = (value / FIXED_POINT_SCALE) + 1;
        } else {
            value /= FIXED_POINT_SCALE;
        }

        // For negative exponents that would result in negative values, set to 0
        // (numbers with negative exponents don't need extra digits)
        if value < 0 {
            table[i] = 0;
        } else {
            table[i] = value as u16;
        }

        i += 1;
    }

    table
}

/// Generates a lookup table of effective precision values for each binary exponent.
///
/// For each exponent, this pre-computes the maximum number of meaningful decimal digits
/// that can be used after accounting for binary-to-decimal conversion overhead.
///
/// This improves performance by avoiding these calculations at runtime and provides
/// consistent precision handling across all floating-point operations.
const fn generate_precision_table() -> [usize; EXPONENT_RANGE] {
    let extra_digits = generate_extra_digits_table();
    let mut table = [0usize; EXPONENT_RANGE];
    let mut i = 0;

    while i < EXPONENT_RANGE {
        // If extra_digits exceeds our precision, result is 0
        if extra_digits[i] > DECIMAL_PRECISION_DIGITS {
            table[i] = 0;
        } else {
            // Store directly as usize to avoid runtime conversions
            table[i] = (DECIMAL_PRECISION_DIGITS - extra_digits[i]) as usize;
        }

        i += 1;
    }

    table
}
