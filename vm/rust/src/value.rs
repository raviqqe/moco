use crate::Integer;
use core::fmt::Debug;

/// A value.
pub trait Value: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord {
    /// A cons.
    type Cons: Integer;

    /// A number.
    type Number: Integer;

    /// A tag.
    type Tag: Integer;

    /// Converts a cons to a value.
    fn from_cons(cons: Self::Cons) -> Self;

    /// Converts a value to a cons.
    fn to_cons(value: Self) -> Self::Cons;

    /// Checks if a value is a cons.
    fn is_cons(value: Self) -> bool;

    /// Converts a number to a value.
    fn from_number(number: Self::Number) -> Self;

    /// Converts a value to a number.
    fn to_number(value: Self) -> Self::Number;
}

/// A 32-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value32(u32);

impl Value for Value32 {
    type Cons = u32;
    type Number = i32;
    type Tag = u8;

    #[inline]
    fn from_cons(cons: Self::Cons) -> Self {
        Self(cons << 1)
    }

    #[inline]
    fn to_cons(value: Self) -> Self::Cons {
        value.0 >> 1
    }

    #[inline]
    fn is_cons(value: Self) -> bool {
        value.0 & 1 == 0
    }

    #[inline]
    fn from_number(number: Self::Number) -> Self {
        Self(((number << 1) | 1) as _)
    }

    #[inline]
    fn to_number(value: Self) -> Self::Number {
        (value.0 >> 1) as _
    }
}

/// A 64-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value64(u64);

impl Value for Value64 {
    type Cons = u64;
    type Number = i64;
    type Tag = u16;

    #[inline]
    fn from_cons(cons: Self::Cons) -> Self {
        Self(cons << 1)
    }

    #[inline]
    fn to_cons(value: Self) -> Self::Cons {
        value.0 >> 1
    }

    #[inline]
    fn is_cons(value: Self) -> bool {
        value.0 & 1 == 0
    }

    #[inline]
    fn from_number(number: Self::Number) -> Self {
        Self(((number << 1) | 1) as _)
    }

    #[inline]
    fn to_number(value: Self) -> Self::Number {
        (value.0 >> 1) as _
    }
}
