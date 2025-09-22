use crate::{Value, integer::Integer};

/// A tag.
pub type Tag = u8;

/// A cons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons<V>(V);

impl<V: Value> Cons<V> {
    /// Creates a cons.
    #[inline]
    pub fn new(index: usize) -> Self {
        Self(V::from_pointer(V::Pointer::from_usize(index) << Tag::BITS))
    }

    /// Returns an index.
    #[inline]
    pub fn index(self) -> usize {
        (self.0.to_pointer() >> Tag::BITS).to_usize()
    }

    /// Returns a tag.
    #[inline]
    pub fn tag(self) -> Tag {
        self.0.to_pointer().to_usize() as _
    }

    /// Sets an index.
    #[inline]
    pub fn set_index(self, index: usize) -> Self {
        self.set_pointer(
            self.0.to_pointer() & Tag::MAX.into() | (V::Pointer::from_usize(index) << Tag::BITS),
        )
    }

    /// Sets a tag.
    #[inline]
    pub fn set_tag(self, tag: Tag) -> Self {
        self.set_pointer(self.0.to_pointer() & !V::Pointer::from(Tag::MAX) | tag.into())
    }

    fn set_pointer(self, pointer: V::Pointer) -> Self {
        Self(self.0.set_pointer(pointer))
    }

    /// Converts a cons to a value.
    #[inline]
    pub(crate) const fn to_value(self) -> V {
        self.0
    }
}

impl<V: Value> From<V> for Cons<V> {
    #[inline]
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
    fn create() {
        assert_eq!(Cons::<Value64>::new(42).index(), 42);
    }

    #[test]
    fn set_index() {
        assert_eq!(Cons::<Value64>::new(0).set_index(42).index(), 42);
    }

    #[test]
    fn set_index_twice() {
        assert_eq!(
            Cons::<Value64>::new(0)
                .set_index(usize::MAX)
                .set_index(42)
                .index(),
            42
        );
    }

    #[test]
    fn set_tag() {
        assert_eq!(Cons::<Value64>::new(0).set_tag(42).tag(), 42);
    }

    #[test]
    fn set_tag_twice() {
        assert_eq!(
            Cons::<Value64>::new(0).set_tag(Tag::MAX).set_tag(42).tag(),
            42
        );
    }
}
