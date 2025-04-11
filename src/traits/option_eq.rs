pub trait OptionEq {
    fn equal_option(&self, other: &Self) -> bool;
}
