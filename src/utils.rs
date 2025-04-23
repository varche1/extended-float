/// Validation helper to check for NaN and infinite values
#[inline(always)]
pub fn is_valid_float<T>(value: T) -> bool
where
    T: num_traits::Float,
{
    !value.is_nan() && !value.is_infinite()
}

/// Checks if a float is invalid (NaN or infinite) and returns a pair of booleans
/// indicating what type of invalid value it is: (is_nan, is_infinite)
#[inline(always)]
pub fn check_invalid_float<T>(value: T) -> Option<(bool, bool)>
where
    T: num_traits::Float,
{
    if value.is_nan() {
        Some((true, false))
    } else if value.is_infinite() {
        Some((false, true))
    } else {
        None
    }
}
