use core::{
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, Div, Mul, Shl, Shr, Sub},
};

/// An integer.
pub trait Integer:
    Add
    + Sub
    + Mul
    + Div
    + Shl
    + Shr
    + BitAnd
    + BitOr
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Clone
    + Copy
    + Default
    + Debug
    + Display
{
    /// A size in bits.
    const BITS: u32;
    /// A mask.
    const MASK: Self;

    /// Converts an integer to `usize`.
    fn to_usize(self) -> usize;
}

macro_rules! impl_integer {
    ($type:ty) => {
        impl Integer for $type {
            const BITS: u32 = Self::BITS;
            const MASK: Self = Self::MAX;

            fn to_usize(self) -> usize {
                self as usize
            }
        }
    };
}

impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
