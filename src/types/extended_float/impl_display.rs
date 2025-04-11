use std::fmt;

use super::ExtendedFloat;
use crate::traits::DisplayableFloat;

impl<T: DisplayableFloat> fmt::Display for ExtendedFloat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}
