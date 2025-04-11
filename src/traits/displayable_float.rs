use std::fmt;

use super::float::Float;

pub trait DisplayableFloat: Float + fmt::Debug + fmt::Display {}

impl<T: Float + fmt::Debug + fmt::Display> DisplayableFloat for T {}
