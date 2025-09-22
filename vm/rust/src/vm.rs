use crate::{Cons, Error, Heap, Memory, Value, instruction::Instruction};

/// A virtual machine.
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
    pub fn run(&self, _program: &[u8]) -> Result<(), Error> {
        loop {
            let instruction = self
                .memory
                .get(Cons::from(self.memory.root()).index() + 1)?;

            match instruction as _ {
                Instruction::Cons => {}
                Instruction::Move => {}
            }
        }

        Ok(())
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
}
