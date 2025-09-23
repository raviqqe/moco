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
        } {}

        Ok(index)
    }

    fn initialize(&self, _program: &[u8]) -> Result<(), Error> {
        // TODO
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value64;

    const HEAP_SIZE: usize = 1 << 8;

    #[test]
    fn index_default() {
        let mut vm = Vm::new([Value64::default(); HEAP_SIZE]).unwrap();

        assert_eq!(vm.memory.get(0b1).unwrap(), Default::default());

        let cons = vm.memory.allocate(1.into(), 2.into()).unwrap();
        vm.memory.set_root(cons.into());

        assert_eq!(vm.memory.get(vm.index(0b10).unwrap()).unwrap(), 1i64.into());
        assert_eq!(vm.memory.get(vm.index(0b11).unwrap()).unwrap(), 2i64.into());
    }
}
