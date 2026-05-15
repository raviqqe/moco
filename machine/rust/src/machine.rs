use crate::{
    Cons, Error, Heap, Integer, Memory, OperationSet, Value, config::INTEGER_BASE,
    instruction::Instruction,
};

/// A virtual machine.
#[derive(Debug)]
pub struct Machine<V, H, const C: usize, O: OperationSet<V, H>> {
    memory: Memory<V, H>,
    #[expect(dead_code)]
    operation_set: O,
}

impl<V: Value, H: Heap<V>, const C: usize, O: OperationSet<V, H>> Machine<V, H, C, O> {
    /// Creates a virtual machine.
    pub fn new(heap: H, operation_set: O) -> Result<Self, Error> {
        Ok(Self {
            memory: Memory::new(heap)?,
            operation_set,
        })
    }

    /// Runs a program.
    pub fn run(&mut self, program: impl IntoIterator<Item = u8>) -> Result<(), Error> {
        self.initialize(program)?;

        while let Ok(mut cons) = self.memory.get(self.index(C)?)?.to_cons() {
            while let Ok(instruction) = self.memory.get(cons.index() + 1)?.to_cons() {
                let operand = self.memory.get(cons.index())?;
                let tag = instruction.tag();
                let index = self.index((tag >> 1) as _)?;

                match tag & 1 {
                    Instruction::CONS => {
                        let cons = self.memory.allocate(operand, self.memory.get(index)?)?;
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

    #[expect(clippy::unused_self)]
    fn initialize(&mut self, bytecode: impl IntoIterator<Item = u8>) -> Result<(), super::Error> {
        let mut bytecode = bytecode.into_iter();

        #[expect(clippy::while_let_on_iterator)]
        while let Some(_byte) = bytecode.next() {}

        Ok(())
    }

    #[expect(dead_code)]
    fn decode_integer_tail(
        bytecode: &mut impl Iterator<Item = u8>,
        mut x: u8,
        mut base: u64,
    ) -> Result<u64, Error> {
        let mut y = (x >> 1) as u64;

        while x & 1 != 0 {
            x = bytecode.next().ok_or(Error::BytecodeEnd)?;
            y += (x as u64 >> 1) * base;
            base *= INTEGER_BASE;
        }

        Ok(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Value64, operation_set::VoidOperationSet};

    const HEAP_SIZE: usize = 1 << 8;

    #[test]
    fn index() {
        let mut machine =
            Machine::<_, _, 0b11, _>::new([Value64::default(); HEAP_SIZE], VoidOperationSet::new())
                .unwrap();

        assert_eq!(machine.memory.get(0b1).unwrap(), Default::default());

        let cons = machine.memory.allocate(1.into(), 2.into()).unwrap();
        machine.memory.set_root(cons.into());

        assert_eq!(machine.memory.get(machine.index(0b10).unwrap()).unwrap(), 1i64.into());
        assert_eq!(machine.memory.get(machine.index(0b11).unwrap()).unwrap(), 2i64.into());
    }
}
