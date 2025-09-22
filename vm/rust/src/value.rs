use crate::Integer;
use crate::cons::Cons;
use core::fmt::Debug;

/// A value.
pub trait Value:
    Clone
    + Copy
    + Debug
    + Default
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + From<Cons<Self>>
    + From<Self::Number>
    + Into<Self::Number>
{
    /// A number.
    type Number: Integer;

    /// A pointer.
    type Pointer: Integer;

    /// Converts a pointer to a value.
    fn from_pointer(pointer: Self::Pointer) -> Self;

    /// Converts a value to a pointer.
    fn to_pointer(self) -> Self::Pointer;

    /// Sets a pointer.
    fn set_pointer(self, pointer: Self::Pointer) -> Self;

    /// Checks if a value is a pointer.
    fn is_pointer(self) -> bool;

    /// Marks a value.
    fn mark(self, mark: bool) -> Self;

    /// Returns `true` if a value is marked.
    fn is_marked(self) -> bool;
}

/// A 16-bit value.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value16(u16);

/// A 32-bit value.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value32(u32);

/// A 64-bit value.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value64(u64);

macro_rules! impl_value {
    ($value:ty, $number:ty, $pointer:ty) => {
        impl Value for $value {
            type Number = $number;
            type Pointer = $pointer;

            #[inline]
            fn from_pointer(pointer: Self::Pointer) -> Self {
                Self(pointer << 2)
            }

            #[inline]
            fn to_pointer(self) -> Self::Pointer {
                self.0 >> 2
            }

            #[inline]
            fn set_pointer(self, pointer: Self::Pointer) -> Self {
                Self(self.0 & 0b11 | (pointer << 2))
            }

            #[inline]
            fn is_pointer(self) -> bool {
                self.0 & 1 == 0
            }

            #[inline]
            fn mark(self, mark: bool) -> Self {
                Self(if mark { self.0 | 0b10 } else { self.0 & !0b10 })
            }

            #[inline]
            fn is_marked(self) -> bool {
                self.0 & 0b10 != 0
            }
        }

        impl From<$number> for $value {
            fn from(number: $number) -> $value {
                Self(((number << 2) | 1) as _)
            }
        }

        impl From<$value> for $number {
            fn from(value: $value) -> $number {
                value.0 as $number >> 2
            }
        }

        impl From<Cons<$value>> for $value {
            fn from(cons: Cons<$value>) -> $value {
                cons.to_value()
            }
        }

        impl Default for $value {
            fn default() -> Self {
                0.into()
            }
        }
    };
}

impl_value!(Value16, i16, u16);
impl_value!(Value32, i32, u32);
impl_value!(Value64, i64, u64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_value {
        ($name:ident, $value:ty) => {
            mod $name {
                use super::*;
                use core::mem::size_of;

                fn from_number(number: <$value as Value>::Number) -> $value {
                    number.into()
                }

                fn to_number(value: $value) -> <$value as Value>::Number {
                    value.into()
                }

                fn from_pointer(pointer: <$value as Value>::Pointer) -> $value {
                    <$value as Value>::from_pointer(pointer)
                }

                #[test]
                fn size() {
                    assert_eq!(
                        size_of::<<$value as Value>::Number>(),
                        size_of::<<$value as Value>::Pointer>()
                    );
                }

                #[test]
                fn zero() {
                    assert_eq!(from_number(0), Default::default());
                }

                #[test]
                fn convert_number() {
                    assert_eq!(to_number(from_number(-42)), -42);
                    assert_eq!(to_number(from_number(-1)), -1);
                    assert_eq!(to_number(from_number(0)), 0);
                    assert_eq!(to_number(from_number(1)), 1);
                    assert_eq!(to_number(from_number(42)), 42);
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

                #[test]
                fn is_marked() {
                    assert!(!from_pointer(0).is_marked());
                    assert!(!from_number(0).is_marked());
                }

                #[test]
                fn mark_number() {
                    assert!(!from_number(0).mark(false).is_marked());
                    assert!(from_number(0).mark(true).is_marked());

                    assert_eq!(to_number(from_number(0).mark(false)), 0);
                    assert_eq!(to_number(from_number(42).mark(false)), 42);
                    assert_eq!(to_number(from_number(-42).mark(false)), -42);
                    assert_eq!(to_number(from_number(0).mark(true)), 0);
                    assert_eq!(to_number(from_number(42).mark(true)), 42);
                    assert_eq!(to_number(from_number(-42).mark(true)), -42);
                }

                #[test]
                fn mark_pointer() {
                    assert!(!from_pointer(0).mark(false).is_marked());
                    assert!(from_pointer(0).mark(true).is_marked());

                    assert_eq!(from_pointer(0).mark(false).to_pointer(), 0);
                    assert_eq!(from_pointer(42).mark(false).to_pointer(), 42);
                    assert_eq!(from_pointer(0).mark(true).to_pointer(), 0);
                    assert_eq!(from_pointer(42).mark(true).to_pointer(), 42);
                }
            }
        };
    }

    test_value!(value16, Value16);
    test_value!(value32, Value32);
    test_value!(value64, Value64);
}
