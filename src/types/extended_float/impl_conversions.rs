use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> From<T> for ExtendedFloat<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: DisplayableFloat> From<ExtendedFloat<T>> for String {
    fn from(value: ExtendedFloat<T>) -> Self {
        value.to_string()
    }
}
