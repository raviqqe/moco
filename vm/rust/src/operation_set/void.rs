use crate::{Memory, OperationSet};

/// A void operation set.
#[derive(Debug, Default)]
pub struct VoidOperationSet {}

impl VoidOperationSet {
    /// Creates an operation set.
    pub const fn new() -> Self {
        Self {}
    }
}

impl<V, H> OperationSet<V, H> for VoidOperationSet {
    type Error = &'static str;

    fn operate(&mut self, _memory: &mut Memory<V, H>, _code: usize) -> Result<(), Self::Error> {
        Err("invalid operation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value64;

    const HEAP_SIZE: usize = 1 << 8;

    #[test]
    fn operate() {
        let mut memory =
            Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

        assert!(VoidOperationSet::new().operate(&mut memory, 0).is_err());
    }
}
