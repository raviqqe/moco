use crate::{Value, integer::Integer};

/// A tag.
pub type Tag = u8;

/// A cons pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons<V>(V);

impl<V: Value> Cons<V> {
    /// Creates a cons pointer.
    pub fn new(index: usize) -> Self {
        Self(V::from_pointer(V::Pointer::from_usize(
            index.to_usize() << Tag::BITS,
        )))
    }

    /// Returns an index.
    pub fn index(self) -> usize {
        self.0.to_pointer().to_usize() >> Tag::BITS as usize
    }

    /// Returns a tag.
    pub fn tag(self) -> Tag {
        self.0.to_pointer().to_usize() as _
    }

    /// Sets a tag.
    pub fn set_tag(self, tag: Tag) -> Self {
        Self(V::from_pointer(self.0.to_pointer() | V::Pointer::from(tag)))
    }

    /// Converts a cons pointer to a value.
    pub const fn to_value(self) -> V {
        self.0
    }
}

impl<V: Value> From<V> for Cons<V> {
    fn from(value: V) -> Self {
        debug_assert!(value.is_pointer());

        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value64;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_tag() {
        assert_eq!(Cons::<Value64>::new(0).set_tag(42).tag(), 42);
    }
}
