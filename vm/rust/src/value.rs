use core::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

/// A value.
pub trait Value: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord {
    /// A cons.
    type Cons: Add
        + Sub
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Clone
        + Copy
        + Default
        + Debug
        + Display;

    /// A number.
    type Number: Add
        + Sub
        + Mul
        + Div
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Clone
        + Copy
        + Default
        + Debug
        + Display;

    /// A tag.
    type Tag: Add
        + Sub
        + Mul
        + Div
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Clone
        + Copy
        + Default
        + Debug
        + Display;

    /// Converts a cons to a value.
    fn from_cons(cons: Self::Cons) -> Self;

    /// Converts a value to a cons.
    fn to_cons(cons: Self) -> Self::Cons;

    /// Checks if a value is a cons.
    fn is_cons(value: Self) -> bool;

    /// Converts a number to a value.
    fn from_number(number: Self::Number) -> Self;

    /// Converts a value to a number.
    fn to_number(number: Self::Number) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct I64Value(u64);

impl Value for I64Value {
    type Number = i64;
    type Tag = u16;

    #[inline]
    fn from_cons(cons: Self) -> Self {
        cons << 1
    }

    #[inline]
    fn to_cons(cons: Self) -> Self {
        cons >> 1
    }

    #[inline]
    fn is_cons(value: Self) -> bool {
        value & 1 == 0
    }

    #[inline]
    fn from_number(number: Self::Number) -> Self {
        (number << 1) | 1
    }

    #[inline]
    fn to_number(number: Self::Number) -> Self {
        number >> 1
    }
}
