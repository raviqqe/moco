use crate::{Memory, OperationSet};

/// An operation set that runs no operation.
#[derive(Clone, Copy, Debug, Default)]
pub struct VoidOperationSet;

impl<V, H> OperationSet<V, H> for VoidOperationSet {
    type Error = ();

    fn operate(&mut self, _memory: &mut Memory<V, H>, _code: usize) -> Result<(), Self::Error> {
        Ok(())
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

        assert_eq!(VoidOperationSet.operate(&mut memory, 0), Ok(()));
    }
}
