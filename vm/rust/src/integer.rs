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
    ($($type:ty),*) => {
        $(
            impl Integer for $type {
                const BITS: u32 = Self::BITS;
                const MASK: Self = Self::MAX;

                fn to_usize(self) -> usize {
                    self as usize
                }
            }
        )*
    };
}

impl_integer!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
