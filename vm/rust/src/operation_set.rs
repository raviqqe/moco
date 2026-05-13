/// An operation set.
pub trait OperationSet {
    fn add(&mut self, operation: String);
    fn contains(&self, operation: &str) -> bool;
}
