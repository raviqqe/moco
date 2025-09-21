use core::{
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub},
};

/// An integer.
pub trait Integer:
    Add
    + Sub
    + Mul
    + Div
    + Shl
    + Shl<u32, Output = Self>
    + Shr
    + Shr<u32, Output = Self>
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Clone
    + Copy
    + Default
    + Debug
    + Display
    + From<u8>
{
    /// A size in bits.
    const BITS: usize;

    /// A mask.
    const MASK: Self;

    /// Converts `usize` to an integer.
    fn from_usize(value: usize) -> Self;

    /// Converts an integer to `usize`.
    fn to_usize(self) -> usize;
}

macro_rules! impl_integer {
    ($type:ty) => {
        impl Integer for $type {
            const BITS: usize = Self::BITS as _;
            const MASK: Self = Self::MAX;

            fn from_usize(value: usize) -> Self {
                value as _
            }

            fn to_usize(self) -> usize {
                self as _
            }
        }
    };
}

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
