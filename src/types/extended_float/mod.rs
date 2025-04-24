mod structs;
pub use structs::ExtendedFloat;

mod impl_conversions;
pub use impl_conversions::ConversionError;

mod impl_checked_ops;
mod impl_display;
mod impl_eq;
mod impl_ops;
mod impl_ord;
