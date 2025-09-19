use crate::Value;

pub struct Cons<V: Value>(V);

impl<V> Cons<V> {
    pub fn new(value: Value) -> Self {
        Self(value)
    }
}
