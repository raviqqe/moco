use crate::{
    Cons, Integer,
    cons::{Cons32, Cons64},
};
use core::fmt::Debug;

/// A value.
pub trait Value: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord {
    /// A cons.
    type Cons: Cons;

    /// A number.
    type Number: Integer;

    /// Converts a cons to a value.
    fn from_cons(cons: Self::Cons) -> Self;

    /// Converts a value to a cons.
    fn to_cons(self) -> Self::Cons;

    /// Checks if a value is a cons.
    fn is_cons(self) -> bool;

    /// Converts a number to a value.
    fn from_number(number: Self::Number) -> Self;

    /// Converts a value to a number.
    fn to_number(self) -> Self::Number;
}

/// A 32-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value32(u32);

/// A 64-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value64(u64);

macro_rules! impl_value {
    ($value:ty, $cons:ty, $number:ty) => {
        impl Value for $value {
            type Cons = $cons;
            type Number = $number;

            #[inline]
            fn from_cons(cons: Self::Cons) -> Self {
                Self(cons.to_raw())
            }

            #[inline]
            fn to_cons(self) -> Self::Cons {
                Self::Cons::from_raw(self.0)
            }

            #[inline]
            fn is_cons(self) -> bool {
                self.0 & 1 == 0
            }

            #[inline]
            fn from_number(number: Self::Number) -> Self {
                Self(((number << 1) | 1) as _)
            }

            #[inline]
            fn to_number(self) -> Self::Number {
                self.0 as Self::Number >> 1
            }
        }
    };
}

impl_value!(Value32, Cons32, i32);
impl_value!(Value64, Cons64, i64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_value {
        ($name:ident, $value:ty) => {
            mod $name {
                use super::*;

                fn cons(index: <<$value as Value>::Cons as Cons>::Raw) -> <$value as Value>::Cons {
                    <$value as Value>::Cons::new(index as _)
                }

                fn from_cons(cons: <$value as Value>::Cons) -> $value {
                    <$value as Value>::from_cons(cons)
                }

                fn from_number(number: <$value as Value>::Number) -> $value {
                    <$value as Value>::from_number(number)
                }

                #[test]
                fn convert_cons() {
                    assert_eq!(from_cons(cons(0)).to_cons().index(), 0);
                    assert_eq!(from_cons(cons(1)).to_cons().index(), 1);
                    assert_eq!(from_cons(cons(42)).to_cons().index(), 42);
                }

                #[test]
                fn check_cons() {
                    assert!(from_cons(cons(0)).is_cons());
                    assert!(!from_number(0).is_cons());
                }

                #[test]
                fn convert_number() {
                    assert_eq!(from_number(-42).to_number(), -42);
                    assert_eq!(from_number(-1).to_number(), -1);
                    assert_eq!(from_number(0).to_number(), 0);
                    assert_eq!(from_number(1).to_number(), 1);
                    assert_eq!(from_number(42).to_number(), 42);
                }
            }
        };
    }

    test_value!(value32, Value32);
    test_value!(value64, Value64);
}
