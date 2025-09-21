use crate::{Cons, error::Error, heap::Heap, value::Value};

/// A memory on a virtual machine.
pub struct Memory<V: Value, H: Heap<V>> {
    heap: H,
    root: V,
    free: V,
}

impl<V: Value, H: Heap<V>> Memory<V, H> {
    /// Creates a memory.
    pub fn new(heap: H) -> Result<Self, Error> {
        let mut this = Self {
            heap,
            root: V::from_number(Default::default()),
            free: Default::default(),
        };

        // TODO Use garbage collection instead.
        this.collect_garbages()?;

        Ok(this)
    }

    /// Returns a root.
    #[inline]
    pub const fn root(&self) -> V {
        self.root
    }

    /// Sets a root.
    #[inline]
    pub const fn set_root(&mut self, value: V) {
        self.root = value;
    }

    fn heap(&self) -> &[V] {
        self.heap.as_ref()
    }

    fn heap_mut(&mut self) -> &mut [V] {
        self.heap.as_mut()
    }

    /// Returns a value at an index.
    #[inline]
    pub fn get(&self, index: usize) -> Result<V, Error> {
        self.heap()
            .get(index)
            .copied()
            .ok_or(Error::InvalidMemoryAccess)
    }

    /// Sets a value at an index.
    #[inline]
    pub fn set(&mut self, index: usize, value: V) -> Result<(), Error> {
        *self
            .heap_mut()
            .get_mut(index)
            .ok_or(Error::InvalidMemoryAccess)? = value;

        Ok(())
    }

    /// Allocates a cons.
    #[inline]
    pub fn allocate(&mut self, car: V, cdr: V) -> Result<Cons<V>, Error> {
        if self.is_out_of_memory() || cfg!(feature = "gc_always") {
            self.collect_garbages()?;
        }

        let cons = Cons::from(self.free);

        self.free = self.get(cons.index() + 1)?.into();
        self.set(cons.index(), car)?;
        self.set(cons.index() + 1, cdr)?;

        Ok(cons)
    }

    fn is_out_of_memory(&self) -> bool {
        !self.free.is_pointer()
    }

    fn collect_garbages(&mut self) -> Result<(), Error> {
        self.mark()?;
        self.sweep()?;

        Ok(())
    }

    fn mark(&mut self) -> Result<(), Error> {
        let mut previous = V::default();
        let mut current = self.root;

        loop {
            if current.is_pointer() && !self.get(Cons::from(current).index())?.is_marked() {
                let cons = Cons::from(current);
                let next = self.get(cons.index())?;
                self.set(cons.index(), previous.mark(true))?;
                previous = current;
                current = next;
            } else if current.is_pointer() && Cons::from(current).index().is_multiple_of(2) {
                let cons = Cons::from(current);
                current = Cons::new(cons.index() + 1).into();
            } else if !previous.is_pointer() {
                break;
            } else {
                let cons = Cons::from(previous);
                previous = self.get(cons.index())?;
                self.set(cons.index(), current)?;
                current = cons.into();
            }
        }

        Ok(())
    }

    fn sweep(&mut self) -> Result<(), Error> {
        self.free = V::default();

        for index in (0..self.heap().len()).step_by(2) {
            let value = self.get(index)?;

            if value.is_marked() {
                self.set(index, value.mark(false))?;
            } else {
                self.set(index + 1, self.free)?;
                self.free = Cons::new(index).into();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value64;

    const HEAP_SIZE: usize = 1 << 10;

    #[test]
    fn create() {
        Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();
    }

    #[test]
    fn allocate() {
        let mut memory = Memory::<Value64, [Value64; 2]>::new([Default::default(); _]).unwrap();

        memory
            .allocate(Default::default(), Default::default())
            .unwrap();
    }
}
