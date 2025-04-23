use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> Sub for ExtendedFloat<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() - other.downgrade())
    }
}

impl<T: DisplayableFloat> SubAssign for ExtendedFloat<T> {
    fn sub_assign(&mut self, other: Self) {
        self.update(self.downgrade() - other.downgrade());
    }
}

impl<T: DisplayableFloat> Add for ExtendedFloat<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() + other.downgrade())
    }
}

impl<T: DisplayableFloat> AddAssign for ExtendedFloat<T> {
    fn add_assign(&mut self, other: Self) {
        self.update(self.downgrade() + other.downgrade());
    }
}

impl<T: DisplayableFloat> Mul for ExtendedFloat<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() * other.downgrade())
    }
}

impl<T: DisplayableFloat> MulAssign for ExtendedFloat<T> {
    fn mul_assign(&mut self, other: Self) {
        self.update(self.downgrade() * other.downgrade());
    }
}

impl<T: DisplayableFloat> Div for ExtendedFloat<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() / other.downgrade())
    }
}

impl<T: DisplayableFloat> DivAssign for ExtendedFloat<T> {
    fn div_assign(&mut self, other: Self) {
        self.update(self.downgrade() / other.downgrade());
    }
}

impl<T: DisplayableFloat> Rem for ExtendedFloat<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.downgrade() % other.downgrade())
    }
}

impl<T: DisplayableFloat> RemAssign for ExtendedFloat<T> {
    fn rem_assign(&mut self, other: Self) {
        self.update(self.downgrade() % other.downgrade());
    }
}

impl<T: DisplayableFloat> Neg for ExtendedFloat<T> {
    type Output = Self;

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
    fn test_zero_division() {
        // Division by zero should result in infinity
        let result = ExtendedFloat::new(5.0) / ExtendedFloat::new(0.0);
        assert!(result.is_infinite());
        assert!(result.is_sign_positive());

        let result = ExtendedFloat::new(-5.0) / ExtendedFloat::new(0.0);
        assert!(result.is_infinite());
        assert!(result.is_sign_negative());

        // Zero divided by zero should result in NaN
        let result = ExtendedFloat::new(0.0) / ExtendedFloat::new(0.0);
        assert!(result.is_nan());
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
    fn test_with_edge_cases() {
        // Test with infinity
        let inf = ExtendedFloat::new(f64::INFINITY);
        let neg_inf = ExtendedFloat::new(f64::NEG_INFINITY);

        assert_eq!(inf + ExtendedFloat::new(5.0), inf);
        assert_eq!(neg_inf + ExtendedFloat::new(5.0), neg_inf);
        assert_eq!(inf - ExtendedFloat::new(5.0), inf);
        assert_eq!(neg_inf - ExtendedFloat::new(5.0), neg_inf);
        assert_eq!(inf * ExtendedFloat::new(5.0), inf);
        assert_eq!(neg_inf * ExtendedFloat::new(5.0), neg_inf);
        assert_eq!(inf * ExtendedFloat::new(-5.0), neg_inf);
        assert_eq!(inf / ExtendedFloat::new(5.0), inf);
        assert_eq!(neg_inf / ExtendedFloat::new(5.0), neg_inf);
        assert_eq!(inf / ExtendedFloat::new(-5.0), neg_inf);
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

        // Test with infinity and NaN
        assert_eq!(
            -ExtendedFloat::new(f64::INFINITY),
            ExtendedFloat::new(f64::NEG_INFINITY)
        );
        assert_eq!(
            -ExtendedFloat::new(f64::NEG_INFINITY),
            ExtendedFloat::new(f64::INFINITY)
        );
        assert!((-ExtendedFloat::new(f64::NAN)).is_nan());
    }

    #[test]
    fn test_zero_remainder() {
        // Remainder by zero should result in NaN
        let result = ExtendedFloat::new(5.0) % ExtendedFloat::new(0.0);
        assert!(result.is_nan());

        // Zero remainder by zero should result in NaN
        let result = ExtendedFloat::new(0.0) % ExtendedFloat::new(0.0);
        assert!(result.is_nan());
    }
}

// TODO: benches
