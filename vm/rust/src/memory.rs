use crate::error::Error;
use crate::{heap::Heap, value::Value};

// TODO
// const CONS_FIELD_COUNT: usize = 2;

/// A memory on a virtual machine.
pub struct Memory<V: Value, H: Heap<V>> {
    root: V::Cons,
    heap: H,
}

impl<V: Value, H: Heap<V>> Memory<V, H> {
    /// Creates a memory.
    pub fn new(heap: H) -> Self {
        Self {
            root: Default::default(),
            heap,
        }
    }

    /// Returns a root.
    #[inline]
    pub const fn root(&self) -> V::Cons {
        self.root
    }

    /// Sets a root.
    #[inline]
    pub const fn set_root(&mut self, cons: V::Cons) {
        self.root = cons;
    }

    /// Returns a value at an index.
    #[inline]
    pub fn get<const G: bool>(&self, index: usize) -> Result<V, Error> {
        self.heap
            .as_ref()
            .get(index)
            .copied()
            .ok_or(Error::InvalidMemoryAccess)
    }

    /// Sets a value at an index.
    #[inline]
    pub fn set<const G: bool>(&mut self, index: usize, value: V) -> Result<(), Error> {
        *self
            .heap
            .as_mut()
            .get_mut(index)
            .ok_or(Error::InvalidMemoryAccess)? = value;

        Ok(())
    }

    /// Allocates a cons.
    #[inline]
    pub fn allocate(&mut self, car: V, cdr: V) -> Result<V::Cons, Error> {
        let mut cons = self.allocate_unchecked(car, cdr)?;

        debug_assert_eq!(cons.tag(), V::Tag::default());
        assert_heap_cons!(self, cons);
        assert_heap_value!(self, car);
        assert_heap_value!(self, cdr);

        if self.is_out_of_memory() || cfg!(feature = "gc_always") {
            self.collect_garbages(Some(&mut cons))?;
        }

        Ok(cons)
    }

    // TODO
    fn is_out_of_memory(&self) -> bool {
        false
    }

    // TODO
    fn collect_garbages(&mut self, cons: Option<&mut V::Cons>) -> Result<(), Error> {
        Ok(())
    }
}
