use crate::{Error, Heap, Memory, Value};

pub struct Vm<V, H> {
    memory: Memory<V, H>,
}

impl<V: Value, H: Heap<V>> Vm<V, H> {
    /// Creates a virtual machine.
    pub fn new(heap: H) -> Result<Self, Error> {
        Ok(Self {
            memory: Memory::new(heap)?,
        })
    }

    /// Runs a virtual machine.
    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
