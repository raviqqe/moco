use crate::{Integer, Value};

/// A cons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons<V: Value, T: Integer = u8> {
    car: V,
    cdr: V,
    tag: T,
    mark: u8,
}

impl<V: Value, T: Integer> Cons<V, T> {
    /// Creates a cons.
    pub const fn new(car: V, cdr: V, tag: T) -> Self {
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
    pub const fn tag(self) -> T {
        self.tag
    }

    /// Returns a mark.
    pub const fn mark(self) -> u8 {
        self.mark
    }
}
