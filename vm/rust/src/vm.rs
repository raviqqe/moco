use crate::{Cons, Error, Heap, Integer, Memory, Value, instruction::Instruction};

/// A virtual machine.
#[derive(Debug)]
pub struct Vm<V, H, const C: usize> {
    memory: Memory<V, H>,
}

impl<V: Value, H: Heap<V>, const C: usize> Vm<V, H, C> {
    /// Creates a virtual machine.
    pub fn new(heap: H) -> Result<Self, Error> {
        Ok(Self {
            memory: Memory::new(heap)?,
        })
    }

    /// Runs a program.
    pub fn run(&mut self, program: &[u8]) -> Result<(), Error> {
        self.initialize(program)?;

        while let Ok(mut cons) = self.memory.get(self.index(C)?)?.to_cons() {
            while let Ok(instruction) = self.memory.get(cons.index() + 1)?.to_cons() {
                let operand = self.memory.get(cons.index())?;
                let tag = instruction.tag();
                let index = self.index((tag >> 1) as _)?;

                match tag & 1 {
                    Instruction::CONS => {
                        let value = self.memory.get(index)?;
                        let cons = self.memory.allocate(operand, value)?;

                        self.memory.set(index, cons.into())?;
                    }
                    instruction => {
                        debug_assert_eq!(instruction, Instruction::MOVE);

                        self.memory.set(
                            index,
                            self.memory.get(
                                operand
                                    .to_number()
                                    .map_err(|_| Error::NumberExpected)?
                                    .to_usize(),
                            )?,
                        )?;
                    }
                }

                cons = instruction;
            }
        }

        Ok(())
    }

    fn index(&self, mut address: usize) -> Result<usize, Error> {
        let mut index = Cons::from(self.memory.root()).index();

        while {
            index += address & 1;
            address >>= 1;
            address > 1
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
        let mut vm = Vm::<_, _, 0b11>::new([Value64::default(); HEAP_SIZE]).unwrap();

        assert_eq!(vm.memory.get(0b1).unwrap(), Default::default());

        let cons = vm.memory.allocate(1.into(), 2.into()).unwrap();
        vm.memory.set_root(cons.into());

        assert_eq!(vm.memory.get(vm.index(0b10).unwrap()).unwrap(), 1i64.into());
        assert_eq!(vm.memory.get(vm.index(0b11).unwrap()).unwrap(), 2i64.into());
    }
}
