use num_traits::{Float as NumFloat, FloatConst};

/// Extended float trait that provides functionality for precise floating-point handling.
///
/// This trait extends the standard floating-point operations with methods that help
/// address common floating-point precision issues, particularly when displaying
/// float values or performing precise calculations.
///
/// Key features:
/// - Accurate binary exponent extraction
/// - Dynamic precision calculation based on value magnitude
/// - Handling of binary-to-decimal conversion overhead
pub trait Float: NumFloat + FloatConst {
    /// Returns the epsilon value for this float type.
    ///
    /// This is the smallest value that can be meaningfully distinguished from zero,
    /// used for floating-point comparisons and zero detection.
    fn epsilon() -> Self;

    /// Returns the binary exponent of the floating point number.
    ///
    /// For IEEE 754 floating-point numbers, this extracts the exponent bits and
    /// adjusts for the bias to get the actual power of 2.
    fn exponent(&self) -> i16;

    /// Returns the decimal precision threshold for this floating point type.
    ///
    /// This represents the magnitude above which precision handling becomes
    /// unnecessary (numbers will be formatted as-is).
    fn decimal_precision() -> Self;

    /// Returns the number of significant decimal digits for this floating point type.
    ///
    /// This is derived from the type's epsilon value and indicates how many
    /// decimal digits can be accurately represented.
    fn decimal_precision_digits() -> u16;

    /// Returns the number of extra decimal digits needed to represent this number.
    ///
    /// This is calculated based on the binary exponent and represents how many
    /// additional decimal places are needed when converting from binary to decimal.
    /// Uses the formula: ceiling(exponent * log10(2))
    fn extra_digits(&self) -> u16;

    /// Returns the effective precision in decimal digits for this number.
    ///
    /// This represents how many meaningful decimal digits can be used after
    /// accounting for the binary-to-decimal conversion overhead.
    /// Formula: max(0, decimal_precision_digits - extra_digits)
    fn precision(&self) -> usize;
}
