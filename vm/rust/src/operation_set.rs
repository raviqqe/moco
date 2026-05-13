use crate::Memory;

/// An operation set.
pub trait OperationSet<V, H> {
    /// An error.
    type Error;

    /// Runs a primitive on a virtual machine.
    fn operate(&mut self, memory: &mut Memory<V, H>, primitive: usize) -> Result<(), Self::Error>;
}
