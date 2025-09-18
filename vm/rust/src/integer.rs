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
    const BITS: u32;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t {
                const BITS: u32 = Self::BITS;
            }
        )*
    };
}

impl_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
