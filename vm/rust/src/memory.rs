use crate::cons::Cons;
use crate::error::Error;
use crate::{heap::Heap, value::Value};

/// A memory on a virtual machine.
pub struct Memory<V: Value, H: Heap<Cons<V>>> {
    root: V::Pointer,
    heap: H,
}

impl<V: Value, H: Heap<Cons<V>>> Memory<V, H> {
    /// Creates a memory.
    pub fn new(heap: H) -> Self {
        Self {
            root: Default::default(),
            heap,
        }
    }

    /// Returns a root.
    #[inline]
    pub const fn root(&self) -> V::Pointer {
        self.root
    }

    /// Sets a root.
    #[inline]
    pub const fn set_root(&mut self, pointer: V::Pointer) {
        self.root = pointer;
    }

    /// Returns a value at an index.
    #[inline]
    pub fn get(&self, index: usize) -> Result<Cons<V>, Error> {
        self.heap
            .as_ref()
            .get(index)
            .copied()
            .ok_or(Error::InvalidMemoryAccess)
    }

    /// Sets a value at an index.
    #[inline]
    pub fn set(&mut self, index: usize, cons: Cons<V>) -> Result<(), Error> {
        *self
            .heap
            .as_mut()
            .get_mut(index)
            .ok_or(Error::InvalidMemoryAccess)? = cons;

        Ok(())
    }

    /// Allocates a cons.
    #[inline]
    pub fn allocate(&mut self, car: V, cdr: V) -> Result<V::Pointer, Error> {
        let mut cons = self.allocate_unchecked(car, cdr)?;

        if self.is_out_of_memory() || cfg!(feature = "gc_always") {
            self.collect_garbages()?;
        }

        Ok(cons)
    }

    // TODO
    #[expect(clippy::unused_self)]
    fn allocate_unchecked(&mut self, _car: V, _cdr: V) -> Result<V::Pointer, Error> {
        Ok(Default::default())
    }

    // TODO
    #[expect(clippy::unused_self)]
    const fn is_out_of_memory(&self) -> bool {
        false
    }

    // TODO
    #[expect(clippy::unused_self)]
    pub const fn collect_garbages(&mut self) -> Result<(), Error> {
        self.mark()?;

        Ok(())
    }

    const fn mark(&mut self) -> Result<(), Error> {
        let mut current = self.root;
        let mut prev = None;

        loop {
            // while current != null && current->markBit == 0 do
            // current->markBit = 1;
            // if current refers to a non-atomic object then
            // next= current->left; current->left= prev;
            // prev= current; current= next;
            // // end of while current
            // // retreat
            // while prev != null && prev->flagBit == 1 do
            // prev->flagBit= 0; next= prev->right;
            // prev->right= current; current= prev;
            // prev= next;
            // // end of while previous
            // if prev == null then
            // return;
            // // switch to right subgraph
            // prev->flagBit= 1;
            // next= prev->left;
            // prev->left= current;
            // current= prev->right;
            // prev->right= next;
        }

        Ok(())
    }
}
