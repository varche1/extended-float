/// Epsilon value for f64, used for floating point comparisons and zero detection.
/// Represents the smallest meaningful difference between two f64 values.
pub const EPSILON: f64 = 1.0e-12;

/// Decimal precision threshold for f64.
/// Numbers with absolute value greater than this are formatted as-is
/// without additional precision handling.
pub const DECIMAL_PRECISION: f64 = 1e15;

/// Number of significant decimal digits that can be accurately represented in f64.
/// This value is derived from f64::EPSILON (2.220446049250313e-16).
/// Though f64 can technically represent 15-17 digits, 15 is used as a conservative
/// value to ensure consistent and accurate results.
pub const DECIMAL_PRECISION_DIGITS: u16 = 15;

/// Minimum exponent for f64 in IEEE 754 representation.
pub const MIN_EXPONENT: i16 = -1023;

/// Maximum exponent for f64 in IEEE 754 representation.
pub const MAX_EXPONENT: i16 = 1024;

/// Total number of possible exponent values for f64.
/// Used for sizing lookup tables that map from exponent to other values.
pub const EXPONENT_RANGE: usize = (MAX_EXPONENT - MIN_EXPONENT + 1) as usize;
