use crate::Value;

type Tag = u8;

/// A cons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons<V: Value> {
    car: V,
    cdr: V,
    tag: Tag,
    mark: u8,
}

impl<V: Value> Cons<V> {
    /// Creates a cons.
    pub const fn new(car: V, cdr: V, tag: Tag) -> Self {
        Self {
            car,
            cdr,
            tag,
            mark: 0,
        }
    }

    /// Returns `car`.
    pub const fn car(self) -> V {
        self.car
    }

    /// Returns `cdr`.
    pub const fn cdr(self) -> V {
        self.cdr
    }

    /// Returns a tag.
    pub const fn tag(self) -> Tag {
        self.tag
    }

    /// Returns a mark.
    pub const fn mark(self) -> u8 {
        self.mark
    }
}
