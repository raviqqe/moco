use crate::Integer;

/// A cons.
pub trait Cons: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord {
    /// A raw value.
    type Raw: Integer;

    /// A tag.
    type Tag: Integer;

    /// Returns a memory index.
    fn index(self) -> Self::Raw;

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

impl Cons for Cons32 {
    type Raw = u32;
    type Tag = u8;

    fn index(self) -> Self::Raw {
        self.0 >> (Self::Tag::BITS + 1)
    }

    fn tag(self) -> Self::Tag {
        ((self.0 >> 1) & Self::Tag::MASK as Self::Raw) as _
    }

    fn to_raw(self) -> Self::Raw {
        self.0
    }

    fn from_raw(raw: Self::Raw) -> Self {
        Self(raw)
    }
}

/// A 64-bit value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons64(u64);

impl Cons for Cons64 {
    type Raw = u64;
    type Tag = u16;

    fn index(self) -> Self::Raw {
        self.0 >> (Self::Tag::BITS + 1)
    }

    fn tag(self) -> Self::Tag {
        ((self.0 >> 1) & Self::Tag::MASK as Self::Raw) as _
    }

    fn to_raw(self) -> Self::Raw {
        self.0
    }

    fn from_raw(raw: Self::Raw) -> Self {
        Self(raw)
    }
}
