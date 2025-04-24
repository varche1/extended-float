use crate::types::extended_float::ConversionError;

/// Trait for safe arithmetic operations that check for invalid results (NaN, infinity)
///
/// This trait provides checked versions of common arithmetic operations that
/// return Option<Self> instead of panicking when the result would be NaN or infinite.
///
/// # Examples
///
/// ```
/// use extended_float::types::ExtendedFloat;
/// use extended_float::traits::CheckedFloatOps;
///
/// let a = ExtendedFloat::new(5.0);
/// let b = ExtendedFloat::new(0.0);
///
/// // Division by zero returns None instead of panicking
/// assert!(a.checked_div(b).is_none());
/// ```
pub trait CheckedFloatOps<Rhs = Self> {
    /// Performs a checked addition, returning None if the result would be NaN or infinite
    fn checked_add(&self, rhs: Rhs) -> Option<Self>
    where
        Self: Sized;

    /// Performs a checked subtraction, returning None if the result would be NaN or infinite
    fn checked_sub(&self, rhs: Rhs) -> Option<Self>
    where
        Self: Sized;

    /// Performs a checked multiplication, returning None if the result would be NaN or infinite
    fn checked_mul(&self, rhs: Rhs) -> Option<Self>
    where
        Self: Sized;

    /// Performs a checked division, returning None if the result would be NaN or infinite
    /// (including division by zero)
    fn checked_div(&self, rhs: Rhs) -> Option<Self>
    where
        Self: Sized;

    /// Performs a checked remainder operation, returning None if the result would be NaN or infinite
    /// (including remainder by zero)
    fn checked_rem(&self, rhs: Rhs) -> Option<Self>
    where
        Self: Sized;

    /// Similar to checked_add but returns a Result with detailed error information
    fn try_add(&self, rhs: Rhs) -> Result<Self, ConversionError>
    where
        Self: Sized;

    /// Similar to checked_sub but returns a Result with detailed error information
    fn try_sub(&self, rhs: Rhs) -> Result<Self, ConversionError>
    where
        Self: Sized;

    /// Similar to checked_mul but returns a Result with detailed error information
    fn try_mul(&self, rhs: Rhs) -> Result<Self, ConversionError>
    where
        Self: Sized;

    /// Similar to checked_div but returns a Result with detailed error information
    fn try_div(&self, rhs: Rhs) -> Result<Self, ConversionError>
    where
        Self: Sized;

    /// Similar to checked_rem but returns a Result with detailed error information
    fn try_rem(&self, rhs: Rhs) -> Result<Self, ConversionError>
    where
        Self: Sized;
}
