use crate::cons::Cons;
use crate::error::Error;
use crate::{heap::Heap, value::Value};

/// A memory on a virtual machine.
pub struct Memory<V: Value, H: Heap<Cons<V>>> {
    root: V,
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
    pub const fn root(&self) -> V {
        self.root
    }

    /// Sets a root.
    #[inline]
    pub const fn set_root(&mut self, pointer: V) {
        self.root = pointer;
    }

    /// Returns a value at an index.
    #[inline]
    pub fn get(&self, index: usize) -> Result<&Cons<V>, Error> {
        self.heap
            .as_ref()
            .get(index)
            .ok_or(Error::InvalidMemoryAccess)
    }

    /// Returns a value at an index.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Result<&mut Cons<V>, Error> {
        self.heap
            .as_mut()
            .get_mut(index)
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
        let cons = self.allocate_unchecked(car, cdr)?;

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
    pub fn collect_garbages(&mut self) -> Result<(), Error> {
        self.mark()?;

        Ok(())
    }

    const fn mark(&mut self) -> Result<(), Error> {
        let mut current = Some(self.root);
        let mut prev = V::default();

        loop {
            while if let Some(current) = current {
                self.get(current)?.mark() == 0
            } else {
                false
            } {
                self.get_mut(current)?.set_mark(1);

                if current.is_cons() {
                    let next = self.get(current)?.car();
                    self.get_mut(current)?.set_cdr(prev);
                    prev = Some(current);
                    current = next;
                }
            }

            // retreat
            // while prev != null && prev->flagBit == 1 do
            // prev->flagBit= 0; next= prev->right;
            // prev->right= current; current= prev;
            // prev= next;
            // // end of while previous
            // if prev == null then
            // return;
            // switch to right subgraph
            // prev->flagBit= 1;
            // next= prev->left;
            // prev->left= current;
            // current= prev->right;
            // prev->right= next;
        }

        Ok(())
    }
}
