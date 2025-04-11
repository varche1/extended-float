use std::ops::{Add, Div, Mul, Sub};

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> Sub for ExtendedFloat<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl<T: DisplayableFloat> Add for ExtendedFloat<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl<T: DisplayableFloat> Mul for ExtendedFloat<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl<T: DisplayableFloat> Div for ExtendedFloat<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};

    use super::*;
    use crate::constants::f64::EPSILON as EPSILON_F64;

    #[test]
    fn test_addition() {
        assert_eq!(ExtendedFloat(5.0) + ExtendedFloat(3.0), ExtendedFloat(8.0));
        assert_eq!(
            ExtendedFloat(-5.0) + ExtendedFloat(3.0),
            ExtendedFloat(-2.0)
        );
        assert_eq!(ExtendedFloat(0.1) + ExtendedFloat(0.2), ExtendedFloat(0.3));

        let small = EPSILON_F64 / 2.0;
        assert_eq!(
            ExtendedFloat(0.0) + ExtendedFloat(small),
            ExtendedFloat(small)
        );

        assert_eq!(
            ExtendedFloat(1e15) + ExtendedFloat(1.0),
            ExtendedFloat(1e15 + 1.0)
        );

        assert_eq!(ExtendedFloat(PI) + ExtendedFloat(E), ExtendedFloat(PI + E));
    }

    #[test]
    fn test_subtraction() {
        // Basic subtraction
        assert_eq!(ExtendedFloat(5.0) - ExtendedFloat(3.0), ExtendedFloat(2.0));
        assert_eq!(
            ExtendedFloat(-5.0) - ExtendedFloat(3.0),
            ExtendedFloat(-8.0)
        );

        // Test with decimals
        assert_eq!(ExtendedFloat(0.3) - ExtendedFloat(0.1), ExtendedFloat(0.2));

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        assert_eq!(
            ExtendedFloat(small) - ExtendedFloat(small),
            ExtendedFloat(0.0)
        );

        // Test with large numbers
        assert_eq!(
            ExtendedFloat(1e15) - ExtendedFloat(1.0),
            ExtendedFloat(1e15 - 1.0)
        );

        // Test with special constants
        assert_eq!(ExtendedFloat(PI) - ExtendedFloat(E), ExtendedFloat(PI - E));
    }

    #[test]
    fn test_multiplication() {
        // Basic multiplication
        assert_eq!(ExtendedFloat(5.0) * ExtendedFloat(3.0), ExtendedFloat(15.0));
        assert_eq!(
            ExtendedFloat(-5.0) * ExtendedFloat(3.0),
            ExtendedFloat(-15.0)
        );
        assert_eq!(
            ExtendedFloat(-5.0) * ExtendedFloat(-3.0),
            ExtendedFloat(15.0)
        );

        // Test with decimals
        assert_eq!(ExtendedFloat(0.1) * ExtendedFloat(0.2), ExtendedFloat(0.02));

        // Test with very small numbers
        let small = EPSILON_F64 / 2.0;
        assert_eq!(
            ExtendedFloat(small) * ExtendedFloat(2.0),
            ExtendedFloat(EPSILON_F64)
        );

        // Test with large numbers
        assert_eq!(ExtendedFloat(1e7) * ExtendedFloat(1e7), ExtendedFloat(1e14));

        // Test with special constants
        assert_eq!(ExtendedFloat(PI) * ExtendedFloat(E), ExtendedFloat(PI * E));

        // Test with zero
        assert_eq!(ExtendedFloat(5.0) * ExtendedFloat(0.0), ExtendedFloat(0.0));
    }

    #[test]
    fn test_division() {
        // Basic division
        assert_eq!(ExtendedFloat(15.0) / ExtendedFloat(3.0), ExtendedFloat(5.0));
        assert_eq!(
            ExtendedFloat(-15.0) / ExtendedFloat(3.0),
            ExtendedFloat(-5.0)
        );
        assert_eq!(
            ExtendedFloat(-15.0) / ExtendedFloat(-3.0),
            ExtendedFloat(5.0)
        );

        // Test with decimals
        assert_eq!(ExtendedFloat(0.1) / ExtendedFloat(0.5), ExtendedFloat(0.2));

        // Test with very small numbers
        let small = EPSILON_F64 * 2.0;
        assert_eq!(
            ExtendedFloat(small) / ExtendedFloat(2.0),
            ExtendedFloat(EPSILON_F64)
        );

        // Test with large numbers
        assert_eq!(ExtendedFloat(1e14) / ExtendedFloat(1e7), ExtendedFloat(1e7));

        // Test with special constants
        assert_eq!(ExtendedFloat(PI) / ExtendedFloat(E), ExtendedFloat(PI / E));

        // Test division by 1
        assert_eq!(ExtendedFloat(5.0) / ExtendedFloat(1.0), ExtendedFloat(5.0));
    }

    #[test]
    fn test_zero_division() {
        // Division by zero should result in infinity
        let result = ExtendedFloat(5.0) / ExtendedFloat(0.0);
        assert!(result.0.is_infinite());
        assert!(result.0.is_sign_positive());

        let result = ExtendedFloat(-5.0) / ExtendedFloat(0.0);
        assert!(result.0.is_infinite());
        assert!(result.0.is_sign_negative());

        // Zero divided by zero should result in NaN
        let result = ExtendedFloat(0.0) / ExtendedFloat(0.0);
        assert!(result.0.is_nan());
    }

    #[test]
    fn test_chain_operations() {
        // Test chaining of multiple operations
        let result = ExtendedFloat(10.0) + ExtendedFloat(5.0) * ExtendedFloat(2.0);
        assert_eq!(result, ExtendedFloat(20.0));

        let result = (ExtendedFloat(10.0) + ExtendedFloat(5.0)) * ExtendedFloat(2.0);
        assert_eq!(result, ExtendedFloat(30.0));

        let result = ExtendedFloat(20.0) - ExtendedFloat(5.0) / ExtendedFloat(5.0);
        assert_eq!(result, ExtendedFloat(19.0));

        let result = (ExtendedFloat(20.0) - ExtendedFloat(5.0)) / ExtendedFloat(5.0);
        assert_eq!(result, ExtendedFloat(3.0));
    }

    #[test]
    fn test_with_edge_cases() {
        // Test with infinity
        let inf = ExtendedFloat(f64::INFINITY);
        let neg_inf = ExtendedFloat(f64::NEG_INFINITY);

        assert_eq!(inf + ExtendedFloat(5.0), inf);
        assert_eq!(neg_inf + ExtendedFloat(5.0), neg_inf);
        assert_eq!(inf - ExtendedFloat(5.0), inf);
        assert_eq!(neg_inf - ExtendedFloat(5.0), neg_inf);
        assert_eq!(inf * ExtendedFloat(5.0), inf);
        assert_eq!(neg_inf * ExtendedFloat(5.0), neg_inf);
        assert_eq!(inf * ExtendedFloat(-5.0), neg_inf);
        assert_eq!(inf / ExtendedFloat(5.0), inf);
        assert_eq!(neg_inf / ExtendedFloat(5.0), neg_inf);
        assert_eq!(inf / ExtendedFloat(-5.0), neg_inf);
    }
}

// TODO: benches
