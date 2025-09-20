use crate::{Value, integer::Integer};

/// A tag.
pub type Tag = u8;

/// A cons pointer.
pub struct Cons<V>(V);

impl<V: Value> Cons<V> {
    /// Creates a cons pointer.
    pub const fn new(index: V::Pointer, tag: Tag) -> Self {
        Self(Value::from_pointer((index << Tag::BITS) | tag as _))
    }

    /// Returns an index.
    pub fn index(self) -> usize {
        self.0.to_pointer().to_usize() >> Tag::BITS as usize
    }

    /// Returns a tag.
    pub fn tag(self) -> Tag {
        self.0.to_pointer().to_usize() as _
    }

    /// Converts a cons pointer to a value.
    pub const fn to_value(self) -> V {
        self.0
    }
}
