use crate::traits::{CheckedFloatOps, DisplayableFloat};
use crate::types::extended_float::{ConversionError, ExtendedFloat};

impl<T: DisplayableFloat> CheckedFloatOps for ExtendedFloat<T> {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        Self::try_new(self.downgrade() + rhs.downgrade())
    }

    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        Self::try_new(self.downgrade() - rhs.downgrade())
    }

    fn checked_mul(&self, rhs: Self) -> Option<Self> {
        Self::try_new(self.downgrade() * rhs.downgrade())
    }

    fn checked_div(&self, rhs: Self) -> Option<Self> {
        Self::try_new(self.downgrade() / rhs.downgrade())
    }

    fn checked_rem(&self, rhs: Self) -> Option<Self> {
        Self::try_new(self.downgrade() % rhs.downgrade())
    }

    fn try_add(&self, rhs: Self) -> Result<Self, ConversionError> {
        Self::try_from_value(self.downgrade() + rhs.downgrade())
    }

    fn try_sub(&self, rhs: Self) -> Result<Self, ConversionError> {
        Self::try_from_value(self.downgrade() - rhs.downgrade())
    }

    fn try_mul(&self, rhs: Self) -> Result<Self, ConversionError> {
        Self::try_from_value(self.downgrade() * rhs.downgrade())
    }

    fn try_div(&self, rhs: Self) -> Result<Self, ConversionError> {
        Self::try_from_value(self.downgrade() / rhs.downgrade())
    }

    fn try_rem(&self, rhs: Self) -> Result<Self, ConversionError> {
        Self::try_from_value(self.downgrade() % rhs.downgrade())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_operations() {
        // Regular cases should succeed
        let a = ExtendedFloat::new(10.0);
        let b = ExtendedFloat::new(2.0);

        assert_eq!(a.checked_add(b).unwrap(), ExtendedFloat::new(12.0));
        assert_eq!(a.checked_sub(b).unwrap(), ExtendedFloat::new(8.0));
        assert_eq!(a.checked_mul(b).unwrap(), ExtendedFloat::new(20.0));
        assert_eq!(a.checked_div(b).unwrap(), ExtendedFloat::new(5.0));
        assert_eq!(a.checked_rem(b).unwrap(), ExtendedFloat::new(0.0));

        // Division by zero should return None
        let zero = ExtendedFloat::new(0.0);
        assert!(a.checked_div(zero).is_none());
        assert!(a.checked_rem(zero).is_none());

        // Overflow cases
        let huge = ExtendedFloat::new(f64::MAX / 2.0);
        assert!(huge.checked_mul(ExtendedFloat::new(3.0)).is_none());
    }

    #[test]
    fn test_try_operations() {
        // Regular cases should succeed
        let a = ExtendedFloat::new(10.0);
        let b = ExtendedFloat::new(2.0);

        assert_eq!(a.try_add(b).unwrap(), ExtendedFloat::new(12.0));
        assert_eq!(a.try_sub(b).unwrap(), ExtendedFloat::new(8.0));
        assert_eq!(a.try_mul(b).unwrap(), ExtendedFloat::new(20.0));
        assert_eq!(a.try_div(b).unwrap(), ExtendedFloat::new(5.0));
        assert_eq!(a.try_rem(b).unwrap(), ExtendedFloat::new(0.0));

        // Division by zero should return an error
        let zero = ExtendedFloat::new(0.0);
        assert_eq!(a.try_div(zero).unwrap_err(), ConversionError::Infinite);
        assert_eq!(a.try_rem(zero).unwrap_err(), ConversionError::NaN);

        // Overflow cases
        let huge = ExtendedFloat::new(f64::MAX / 2.0);
        assert_eq!(
            huge.try_mul(ExtendedFloat::new(3.0)).unwrap_err(),
            ConversionError::Infinite
        );
    }
}
