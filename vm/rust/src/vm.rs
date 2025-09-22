use crate::{Error, Heap, Memory, Value};

pub struct Vm<V, H> {
    memory: Memory<V, H>,
}

impl<V: Value, H: Heap<V>> Vm<V, H> {
    pub fn new(heap: H) -> Result<Self, Error> {
        Ok(Self {
            memory: Memory::new(heap)?,
        })
    }
}
