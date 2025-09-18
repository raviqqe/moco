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
            self.collect_garbages(Some(&mut cons))?;
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
    const fn collect_garbages(&mut self, _cons: Option<&mut V::Pointer>) -> Result<(), Error> {
        Ok(())
    }
}
