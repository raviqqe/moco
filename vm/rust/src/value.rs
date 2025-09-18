use crate::Integer;
use core::fmt::Debug;

/// A value.
pub trait Value: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord {
    /// A number.
    type Number: Integer;

    /// A pointer.
    type Pointer: Integer;

    /// Converts a number to a value.
    fn from_number(number: Self::Number) -> Self;

    /// Converts a value to a number.
    fn to_number(self) -> Self::Number;

    /// Converts a pointer to a value.
    fn from_pointer(cons: Self::Pointer) -> Self;

    /// Converts a value to a pointer.
    fn to_pointer(self) -> Self::Pointer;

    /// Checks if a value is a pointer.
    fn is_pointer(self) -> bool;
}

/// A 32-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value32(u32);

/// A 64-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value64(u64);

macro_rules! impl_value {
    ($value:ty, $number:ty, $pointer:ty) => {
        impl Value for $value {
            type Number = $number;
            type Pointer = $pointer;

            #[inline]
            fn from_number(number: Self::Number) -> Self {
                Self(((number << 1) | 1) as _)
            }

            #[inline]
            fn to_number(self) -> Self::Number {
                self.0 as Self::Number >> 1
            }

            #[inline]
            fn from_pointer(pointer: Self::Pointer) -> Self {
                Self(pointer << 1)
            }

            #[inline]
            fn to_pointer(self) -> Self::Pointer {
                self.0 >> 1
            }

            #[inline]
            fn is_pointer(self) -> bool {
                self.0 & 1 == 0
            }
        }
    };
}

impl_value!(Value32, i32, u32);
impl_value!(Value64, i64, u64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_value {
        ($name:ident, $value:ty) => {
            mod $name {
                use super::*;

                fn from_number(number: <$value as Value>::Number) -> $value {
                    <$value as Value>::from_number(number)
                }

                fn from_pointer(pointer: <$value as Value>::Pointer) -> $value {
                    <$value as Value>::from_pointer(pointer)
                }

                #[test]
                fn convert_number() {
                    assert_eq!(from_number(-42).to_number(), -42);
                    assert_eq!(from_number(-1).to_number(), -1);
                    assert_eq!(from_number(0).to_number(), 0);
                    assert_eq!(from_number(1).to_number(), 1);
                    assert_eq!(from_number(42).to_number(), 42);
                }

                #[test]
                fn convert_pointer() {
                    assert_eq!(from_pointer(0).to_pointer(), 0);
                    assert_eq!(from_pointer(1).to_pointer(), 1);
                    assert_eq!(from_pointer(42).to_pointer(), 42);
                }

                #[test]
                fn check_pointer() {
                    assert!(from_pointer(0).is_pointer());
                    assert!(!from_number(0).is_pointer());
                }
            }
        };
    }

    test_value!(value32, Value32);
    test_value!(value64, Value64);
}
