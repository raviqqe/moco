use core::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

/// A value.
pub trait Value {
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
}

impl Value for i64 {}
