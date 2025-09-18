use crate::Integer;

/// A cons.
pub trait Cons: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord {
    /// A raw value.
    type Raw: Integer;

    /// A tag.
    type Tag: Integer;

    /// Creates a cons.
    fn new(index: usize) -> Self;

    /// Returns a memory index.
    fn index(self) -> usize;

    /// Returns a tag.
    fn tag(self) -> Self::Tag;

    /// Converts a cons to a raw value.
    fn to_raw(self) -> Self::Raw;

    /// Converts a raw value to a cons.
    fn from_raw(raw: Self::Raw) -> Self;
}

/// A 32-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons32(u32);

/// A 64-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons64(u64);

macro_rules! impl_cons {
    ($cons:ty, $raw:ty, $tag:ty) => {
        impl Cons for $cons {
            type Raw = $raw;
            type Tag = $tag;

            #[inline]
            fn new(index: usize) -> Self {
                Self((index as Self::Raw) << (Self::Tag::BITS + 1))
            }

            #[inline]
            fn index(self) -> usize {
                (self.0 >> (Self::Tag::BITS + 1)) as _
            }

            #[inline]
            fn tag(self) -> Self::Tag {
                (self.0 >> 1) as Self::Tag & Self::Tag::MASK
            }

            #[inline]
            fn to_raw(self) -> Self::Raw {
                self.0
            }

            #[inline]
            fn from_raw(raw: Self::Raw) -> Self {
                Self(raw)
            }
        }
    };
}

impl_cons!(Cons32, u32, u8);
impl_cons!(Cons64, u64, u16);
