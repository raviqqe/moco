use crate::Memory;

/// An operation set.
pub trait OperationSet<V, H> {
    /// An error.
    type Error;

    /// Runs an operation.
    fn operate(&mut self, memory: &mut Memory<V, H>, code: usize) -> Result<(), Self::Error>;
}
