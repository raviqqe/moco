use crate::error::Error;

const CONS_FIELD_COUNT: usize = 2;

/// A memory on a virtual machine.
pub struct Memory<'a> {
    root: Cons,
    allocation_index: usize,
    space: bool,
    heap: &'a mut [Value],
}

impl<'a> Memory<'a> {
    /// Creates a memory.
    pub fn new(heap: &'a mut [Value]) -> Self {
        Self {
            root: NEVER,
            allocation_index: 0,
            space: false,
            heap,
        }
    }

    /// Returns a root.
    #[inline]
    pub const fn root(&self) -> Cons {
        self.root
    }

    // Garbage collection

    /// Collects garbage memory blocks.
    pub fn collect_garbages(&mut self, cons: Option<&mut Cons>) -> Result<(), Error> {
        self.allocation_index = 0;
        self.space = !self.space;

        self.code = self.copy_cons(self.code)?;
        self.stack = self.copy_cons(self.stack)?;
        self.r#false = self.copy_cons(self.r#false)?;
        self.register = self.copy_cons(self.register)?;

        if let Some(cons) = cons {
            *cons = self.copy_cons(*cons)?;
        }

        let mut index = self.allocation_start();

        while index < self.allocation_end() {
            let value = self.copy_value(self.get::<false>(index)?)?;
            self.set::<false>(index, value)?;
            index += 1;
        }

        Ok(())
    }

    fn copy_value(&mut self, value: Value) -> Result<Value, Error> {
        Ok(if let Some(cons) = value.to_cons() {
            self.copy_cons(cons)?.into()
        } else {
            value
        })
    }

    fn copy_cons(&mut self, cons: Cons) -> Result<Cons, Error> {
        Ok(if cons == NEVER {
            NEVER
        } else if self.garbage_car(cons)? == NEVER.into() {
            // Get a forward pointer.
            self.garbage_cdr(cons)?.assume_cons()
        } else {
            let copy = self.allocate_unchecked(self.garbage_car(cons)?, self.garbage_cdr(cons)?)?;

            // Set a forward pointer.
            self.set_garbage_car(cons, NEVER.into())?;
            self.set_garbage_cdr(cons, copy.into())?;

            copy
        }
        .set_tag(cons.tag()))
    }
}
