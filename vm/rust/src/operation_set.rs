/// An operation set.
pub trait OperationSet {
    /// An error.
    type Error: Exception;

    /// Runs a primitive on a virtual machine.
    #[maybe_async]
    fn operate(&mut self, memory: &mut Memory<H>, primitive: usize) -> Result<(), Self::Error>;
}
