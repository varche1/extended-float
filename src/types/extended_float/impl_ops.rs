use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> Sub for ExtendedFloat<T> {
    type Output = Self;

    /// Subtracts two ExtendedFloat values.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., overflow)
    ///
    /// For a non-panicking version, use `checked_sub` or `try_sub`.
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() - other.downgrade())
    }
}

impl<T: DisplayableFloat> SubAssign for ExtendedFloat<T> {
    /// Subtracts another ExtendedFloat value from this one.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., overflow)
    fn sub_assign(&mut self, other: Self) {
        self.update(self.downgrade() - other.downgrade());
    }
}

impl<T: DisplayableFloat> Add for ExtendedFloat<T> {
    type Output = Self;

    /// Adds two ExtendedFloat values.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., overflow)
    ///
    /// For a non-panicking version, use `checked_add` or `try_add`.
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() + other.downgrade())
    }
}

impl<T: DisplayableFloat> AddAssign for ExtendedFloat<T> {
    /// Adds another ExtendedFloat value to this one.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., overflow)
    fn add_assign(&mut self, other: Self) {
        self.update(self.downgrade() + other.downgrade());
    }
}

impl<T: DisplayableFloat> Mul for ExtendedFloat<T> {
    type Output = Self;

    /// Multiplies two ExtendedFloat values.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., overflow)
    ///
    /// For a non-panicking version, use `checked_mul` or `try_mul`.
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() * other.downgrade())
    }
}

impl<T: DisplayableFloat> MulAssign for ExtendedFloat<T> {
    /// Multiplies this ExtendedFloat value by another.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., overflow)
    fn mul_assign(&mut self, other: Self) {
        self.update(self.downgrade() * other.downgrade());
    }
}

impl<T: DisplayableFloat> Div for ExtendedFloat<T> {
    type Output = Self;

    /// Divides two ExtendedFloat values.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., division by zero)
    ///
    /// For a non-panicking version, use `checked_div` or `try_div`.
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() / other.downgrade())
    }
}

impl<T: DisplayableFloat> DivAssign for ExtendedFloat<T> {
    /// Divides this ExtendedFloat value by another.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., division by zero)
    fn div_assign(&mut self, other: Self) {
        self.update(self.downgrade() / other.downgrade());
    }
}

impl<T: DisplayableFloat> Rem for ExtendedFloat<T> {
    type Output = Self;

    /// Calculates the remainder of the division of two ExtendedFloat values.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., remainder by zero)
    ///
    /// For a non-panicking version, use `checked_rem` or `try_rem`.
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() % other.downgrade())
    }
}

impl<T: DisplayableFloat> RemAssign for ExtendedFloat<T> {
    /// Sets this ExtendedFloat value to the remainder of division by another.
    ///
    /// # Panics
    ///
    /// This operation will panic if:
    /// - Either operand contains an invalid value (NaN, infinity)
    /// - The result would be NaN or infinite (e.g., remainder by zero)
    fn rem_assign(&mut self, other: Self) {
        self.update(self.downgrade() % other.downgrade());
    }
}

impl<T: DisplayableFloat> Neg for ExtendedFloat<T> {
    type Output = Self;

    /// Negates this ExtendedFloat value.
    ///
    /// # Panics
    ///
    /// This operation will panic if the operand contains an invalid value (NaN, infinity).
    fn neg(self) -> Self::Output {
        Self::new(-self.downgrade())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};

    use super::*;
    use crate::constants::f64::EPSILON as EPSILON_F64;

    #[test]
    fn test_addition() {
        assert_eq!(
            ExtendedFloat::new(5.0) + ExtendedFloat::new(3.0),
            ExtendedFloat::new(8.0)
        );
        assert_eq!(
            ExtendedFloat::new(-5.0) + ExtendedFloat::new(3.0),
            ExtendedFloat::new(-2.0)
        );
        assert_eq!(
            ExtendedFloat::new(0.1) + ExtendedFloat::new(0.2),
            ExtendedFloat::new(0.3)
        );

        let small = EPSILON_F64 / 2.0;
        assert_eq!(
            ExtendedFloat::new(0.0) + ExtendedFloat::new(small),
            ExtendedFloat::new(small)
        );

        assert_eq!(
            ExtendedFloat::new(1e15) + ExtendedFloat::new(1.0),
            ExtendedFloat::new(1e15 + 1.0)
        );

        assert_eq!(
            ExtendedFloat::new(PI) + ExtendedFloat::new(E),
            ExtendedFloat::new(PI + E)
        );
    }

    #[test]
    fn test_subtraction() {
        // Basic subtraction
        assert_eq!(
            ExtendedFloat::new(5.0) - ExtendedFloat::new(3.0),
            ExtendedFloat::new(2.0)
        );
        assert_eq!(
            ExtendedFloat::new(-5.0) - ExtendedFloat::new(3.0),
            ExtendedFloat::new(-8.0)
        );

        // Test with decimals
        assert_eq!(
            ExtendedFloat::new(0.3) - ExtendedFloat::new(0.1),
            ExtendedFloat::new(0.2)
        );

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        assert_eq!(
            ExtendedFloat::new(small) - ExtendedFloat::new(small),
            ExtendedFloat::new(0.0)
        );

        // Test with large numbers
        assert_eq!(
            ExtendedFloat::new(1e15) - ExtendedFloat::new(1.0),
            ExtendedFloat::new(1e15 - 1.0)
        );

        // Test with special constants
        assert_eq!(
            ExtendedFloat::new(PI) - ExtendedFloat::new(E),
            ExtendedFloat::new(PI - E)
        );
    }

    #[test]
    fn test_multiplication() {
        // Basic multiplication
        assert_eq!(
            ExtendedFloat::new(5.0) * ExtendedFloat::new(3.0),
            ExtendedFloat::new(15.0)
        );
        assert_eq!(
            ExtendedFloat::new(-5.0) * ExtendedFloat::new(3.0),
            ExtendedFloat::new(-15.0)
        );
        assert_eq!(
            ExtendedFloat::new(-5.0) * ExtendedFloat::new(-3.0),
            ExtendedFloat::new(15.0)
        );

        // Test with decimals
        assert_eq!(
            ExtendedFloat::new(0.1) * ExtendedFloat::new(0.2),
            ExtendedFloat::new(0.02)
        );

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        assert_eq!(
            ExtendedFloat::new(small) * ExtendedFloat::new(2.0),
            ExtendedFloat::new(EPSILON_F64)
        );

        // Test with large numbers
        assert_eq!(
            ExtendedFloat::new(1e7) * ExtendedFloat::new(1e7),
            ExtendedFloat::new(1e14)
        );

        // Test with special constants
        assert_eq!(
            ExtendedFloat::new(PI) * ExtendedFloat::new(E),
            ExtendedFloat::new(PI * E)
        );

        // Test with zero
        assert_eq!(
            ExtendedFloat::new(5.0) * ExtendedFloat::new(0.0),
            ExtendedFloat::new(0.0)
        );
    }

    #[test]
    fn test_division() {
        // Basic division
        assert_eq!(
            ExtendedFloat::new(15.0) / ExtendedFloat::new(3.0),
            ExtendedFloat::new(5.0)
        );
        assert_eq!(
            ExtendedFloat::new(-15.0) / ExtendedFloat::new(3.0),
            ExtendedFloat::new(-5.0)
        );
        assert_eq!(
            ExtendedFloat::new(-15.0) / ExtendedFloat::new(-3.0),
            ExtendedFloat::new(5.0)
        );

        // Test with decimals
        assert_eq!(
            ExtendedFloat::new(0.1) / ExtendedFloat::new(0.5),
            ExtendedFloat::new(0.2)
        );

        // Test with very small numbers
        let small = EPSILON_F64 * 2.0;
        assert_eq!(
            ExtendedFloat::new(small) / ExtendedFloat::new(2.0),
            ExtendedFloat::new(EPSILON_F64)
        );

        // Test with large numbers
        assert_eq!(
            ExtendedFloat::new(1e14) / ExtendedFloat::new(1e7),
            ExtendedFloat::new(1e7)
        );

        // Test with special constants
        assert_eq!(
            ExtendedFloat::new(PI) / ExtendedFloat::new(E),
            ExtendedFloat::new(PI / E)
        );

        // Test division by 1
        assert_eq!(
            ExtendedFloat::new(5.0) / ExtendedFloat::new(1.0),
            ExtendedFloat::new(5.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        let _ = ExtendedFloat::new(5.0) / ExtendedFloat::new(0.0);
    }

    #[test]
    #[should_panic]
    fn test_zero_division_by_zero() {
        let _ = ExtendedFloat::new(0.0) / ExtendedFloat::new(0.0);
    }

    #[test]
    fn test_chain_operations() {
        // Test chaining of multiple operations
        let result = ExtendedFloat::new(10.0) + ExtendedFloat::new(5.0) * ExtendedFloat::new(2.0);
        assert_eq!(result, ExtendedFloat::new(20.0));

        let result = (ExtendedFloat::new(10.0) + ExtendedFloat::new(5.0)) * ExtendedFloat::new(2.0);
        assert_eq!(result, ExtendedFloat::new(30.0));

        let result = ExtendedFloat::new(20.0) - ExtendedFloat::new(5.0) / ExtendedFloat::new(5.0);
        assert_eq!(result, ExtendedFloat::new(19.0));

        let result = (ExtendedFloat::new(20.0) - ExtendedFloat::new(5.0)) / ExtendedFloat::new(5.0);
        assert_eq!(result, ExtendedFloat::new(3.0));
    }

    #[test]
    #[should_panic]
    fn test_with_edge_cases_add() {
        // Test with infinity
        let inf = unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) };
        let _ = inf + ExtendedFloat::new(5.0);
    }

    #[test]
    #[should_panic]
    fn test_with_edge_cases_sub() {
        // Test with infinity
        let inf = unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) };
        let _ = inf - ExtendedFloat::new(5.0);
    }

    #[test]
    #[should_panic]
    fn test_with_edge_cases_mul() {
        // Test with infinity
        let inf = unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) };
        let _ = inf * ExtendedFloat::new(5.0);
    }

    #[test]
    #[should_panic]
    fn test_with_edge_cases_div() {
        // Test with infinity
        let inf = unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) };
        let _ = inf / ExtendedFloat::new(5.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = ExtendedFloat::new(5.0);
        a += ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(8.0));

        // Test with negative values
        let mut a = ExtendedFloat::new(5.0);
        a += ExtendedFloat::new(-3.0);
        assert_eq!(a, ExtendedFloat::new(2.0));

        // Test with decimals
        let mut a = ExtendedFloat::new(0.1);
        a += ExtendedFloat::new(0.2);
        assert_eq!(a, ExtendedFloat::new(0.3));

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        let mut a = ExtendedFloat::new(0.0);
        a += ExtendedFloat::new(small);
        assert_eq!(a, ExtendedFloat::new(small));

        // Test with large numbers
        let mut a = ExtendedFloat::new(1e15);
        a += ExtendedFloat::new(1.0);
        assert_eq!(a, ExtendedFloat::new(1e15 + 1.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = ExtendedFloat::new(5.0);
        a -= ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(2.0));

        // Test with negative values
        let mut a = ExtendedFloat::new(5.0);
        a -= ExtendedFloat::new(-3.0);
        assert_eq!(a, ExtendedFloat::new(8.0));

        // Test with decimals
        let mut a = ExtendedFloat::new(0.3);
        a -= ExtendedFloat::new(0.1);
        assert_eq!(a, ExtendedFloat::new(0.2));

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        let mut a = ExtendedFloat::new(small);
        a -= ExtendedFloat::new(small);
        assert_eq!(a, ExtendedFloat::new(0.0));

        // Test with large numbers
        let mut a = ExtendedFloat::new(1e15);
        a -= ExtendedFloat::new(1.0);
        assert_eq!(a, ExtendedFloat::new(1e15 - 1.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = ExtendedFloat::new(5.0);
        a *= ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(15.0));

        // Test with negative values
        let mut a = ExtendedFloat::new(5.0);
        a *= ExtendedFloat::new(-3.0);
        assert_eq!(a, ExtendedFloat::new(-15.0));

        let mut a = ExtendedFloat::new(-5.0);
        a *= ExtendedFloat::new(-3.0);
        assert_eq!(a, ExtendedFloat::new(15.0));

        // Test with decimals
        let mut a = ExtendedFloat::new(0.1);
        a *= ExtendedFloat::new(0.2);
        assert_eq!(a, ExtendedFloat::new(0.02));

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        let mut a = ExtendedFloat::new(small);
        a *= ExtendedFloat::new(2.0);
        assert_eq!(a, ExtendedFloat::new(EPSILON_F64));

        // Test with large numbers
        let mut a = ExtendedFloat::new(1e7);
        a *= ExtendedFloat::new(1e7);
        assert_eq!(a, ExtendedFloat::new(1e14));

        // Test with zero
        let mut a = ExtendedFloat::new(5.0);
        a *= ExtendedFloat::new(0.0);
        assert_eq!(a, ExtendedFloat::new(0.0));
    }

    #[test]
    fn test_div_assign() {
        let mut a = ExtendedFloat::new(15.0);
        a /= ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(5.0));

        // Test with negative values
        let mut a = ExtendedFloat::new(-15.0);
        a /= ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(-5.0));

        let mut a = ExtendedFloat::new(-15.0);
        a /= ExtendedFloat::new(-3.0);
        assert_eq!(a, ExtendedFloat::new(5.0));

        // Test with decimals
        let mut a = ExtendedFloat::new(0.1);
        a /= ExtendedFloat::new(0.5);
        assert_eq!(a, ExtendedFloat::new(0.2));

        // Test with very small numbers
        let small = EPSILON_F64 * 2.0;
        let mut a = ExtendedFloat::new(small);
        a /= ExtendedFloat::new(2.0);
        assert_eq!(a, ExtendedFloat::new(EPSILON_F64));

        // Test with large numbers
        let mut a = ExtendedFloat::new(1e14);
        a /= ExtendedFloat::new(1e7);
        assert_eq!(a, ExtendedFloat::new(1e7));

        // Test division by 1
        let mut a = ExtendedFloat::new(5.0);
        a /= ExtendedFloat::new(1.0);
        assert_eq!(a, ExtendedFloat::new(5.0));
    }

    #[test]
    fn test_remainder() {
        // Basic remainder
        assert_eq!(
            ExtendedFloat::new(7.0) % ExtendedFloat::new(3.0),
            ExtendedFloat::new(1.0)
        );
        assert_eq!(
            ExtendedFloat::new(8.0) % ExtendedFloat::new(2.5),
            ExtendedFloat::new(0.5)
        );

        // Test with negative values - follows Rust's remainder behavior
        assert_eq!(
            ExtendedFloat::new(-7.0) % ExtendedFloat::new(3.0),
            ExtendedFloat::new(-1.0)
        );
        assert_eq!(
            ExtendedFloat::new(7.0) % ExtendedFloat::new(-3.0),
            ExtendedFloat::new(1.0)
        );
        assert_eq!(
            ExtendedFloat::new(-7.0) % ExtendedFloat::new(-3.0),
            ExtendedFloat::new(-1.0)
        );

        // Test with decimals
        assert_eq!(
            ExtendedFloat::new(0.7) % ExtendedFloat::new(0.3),
            ExtendedFloat::new(0.1)
        );

        // Test with very small numbers
        let small = EPSILON_F64 * 5.0;
        assert_eq!(
            ExtendedFloat::new(small) % ExtendedFloat::new(EPSILON_F64 * 2.0),
            ExtendedFloat::new(EPSILON_F64)
        );

        // Test with large numbers
        assert_eq!(
            ExtendedFloat::new(1e14 + 3.0) % ExtendedFloat::new(5.0),
            ExtendedFloat::new(3.0)
        );
    }

    #[test]
    fn test_remainder_assign() {
        // Basic remainder assignment
        let mut a = ExtendedFloat::new(7.0);
        a %= ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(1.0));

        // Test with negative values
        let mut a = ExtendedFloat::new(-7.0);
        a %= ExtendedFloat::new(3.0);
        assert_eq!(a, ExtendedFloat::new(-1.0));

        let mut a = ExtendedFloat::new(7.0);
        a %= ExtendedFloat::new(-3.0);
        assert_eq!(a, ExtendedFloat::new(1.0));

        // Test with decimals
        let mut a = ExtendedFloat::new(0.7);
        a %= ExtendedFloat::new(0.3);
        assert_eq!(a, ExtendedFloat::new(0.1));

        // Test with very small numbers
        let small = EPSILON_F64 * 5.0;
        let mut a = ExtendedFloat::new(small);
        a %= ExtendedFloat::new(EPSILON_F64 * 2.0);
        assert_eq!(a, ExtendedFloat::new(EPSILON_F64));

        // Test with large numbers
        let mut a = ExtendedFloat::new(1e14 + 3.0);
        a %= ExtendedFloat::new(5.0);
        assert_eq!(a, ExtendedFloat::new(3.0));
    }

    #[test]
    fn test_negation() {
        // Basic negation
        assert_eq!(-ExtendedFloat::new(5.0), ExtendedFloat::new(-5.0));
        assert_eq!(-ExtendedFloat::new(-5.0), ExtendedFloat::new(5.0));
        assert_eq!(-ExtendedFloat::new(0.0), ExtendedFloat::new(-0.0));

        // Test with decimals
        assert_eq!(-ExtendedFloat::new(0.1), ExtendedFloat::new(-0.1));

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        assert_eq!(-ExtendedFloat::new(small), ExtendedFloat::new(-small));

        // Test with large numbers
        assert_eq!(-ExtendedFloat::new(1e15), ExtendedFloat::new(-1e15));

        // Test with special constants
        assert_eq!(-ExtendedFloat::new(PI), ExtendedFloat::new(-PI));
    }

    #[test]
    #[should_panic]
    fn test_zero_remainder() {
        let result = ExtendedFloat::new(5.0) % ExtendedFloat::new(0.0);
        assert!(result.is_nan());
    }

    #[test]
    #[should_panic]
    fn test_modulo_by_zero() {
        let _ = ExtendedFloat::new(5.0) % ExtendedFloat::new(0.0);
    }
}

// TODO: benches
