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
}

impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
