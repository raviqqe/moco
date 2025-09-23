use crate::{Cons, Error, Heap, Memory, Value, instruction::Instruction};

const CODE: usize = 0b101;

/// A virtual machine.
#[derive(Debug)]
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

    /// Runs a program.
    pub fn run(&self, program: &[u8]) -> Result<(), Error> {
        self.initialize(program)?;

        loop {
            let instruction = self.memory.get(self.index(CODE)?)?;

            match Cons::from(instruction).tag() {
                Instruction::CONS => {}
                instruction => {
                    debug_assert_eq!(instruction, Instruction::MOVE);
                }
            }
        }
    }

    fn index(&self, mut address: usize) -> Result<usize, Error> {
        let mut index = Cons::from(self.memory.root()).index();

        while {
            index += address & 1;
            address >>= 1;
            address != 1
        } {
            Cons::from(self.memory.get(index)?).index();
        }

        Ok(index)
    }

    fn initialize(&self, _program: &[u8]) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value64;

    const HEAP_SIZE: usize = 1 << 8;

    #[test]
    fn index() {
        let vm = Vm::new([Value64::default(); HEAP_SIZE]).unwrap();

        assert_eq!(vm.memory.get().unwrap(), Default::default());
    }
}
