use core::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

/// A value.
pub trait Value {
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
    fn from_cons(cons: Self) -> Self;

    /// Converts a value to a cons.
    fn to_cons(cons: Self) -> Self;

    /// Checks if a value is a cons.
    fn is_cons(value: Self) -> bool;

    /// Converts a number to a value.
    fn from_number(number: Self::Number) -> Self;

    /// Converts a value to a number.
    fn to_number(number: Self::Number) -> Self;

    fn from_raw(raw: Self) -> Self::Number;
    fn to_raw(number: Self::Number) -> Self;
}

impl Value for i64 {
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

    #[inline]
    fn from_i64(number: i64) -> Self::Number {
        from_number(number)
    }

    #[inline]
    fn to_i64(number: Self::Number) -> i64 {
        to_number(number)
    }

    #[inline]
    fn from_raw(raw: Self) -> Self::Number {
        raw as _
    }

    #[inline]
    fn to_raw(number: Self::Number) -> Self {
        number as _
    }
}
