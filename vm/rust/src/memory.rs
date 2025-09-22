use crate::{Cons, error::Error, heap::Heap, value::Value};

/// A memory on a virtual machine.
#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
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
            root: Default::default(),
            free: Default::default(),
        };

        this.collect_garbages()?;

        Ok(this)
    }

    fn heap(&self) -> &[V] {
        self.heap.as_ref()
    }

    fn heap_mut(&mut self) -> &mut [V] {
        self.heap.as_mut()
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

        self.free = self.get(cons.index() + 1)?;
        self.set(cons.index(), car)?;
        self.set(cons.index() + 1, cdr)?;

        Ok(cons)
    }

    #[inline]
    fn is_out_of_memory(&self) -> bool {
        !self.free.is_pointer()
    }

    pub(crate) fn collect_garbages(&mut self) -> Result<(), Error> {
        self.mark()?;
        self.sweep()?;

        Ok(())
    }

    fn mark(&mut self) -> Result<(), Error> {
        if !self.root.is_pointer() {
            return Ok(());
        }

        let mut previous = V::default();
        let mut current = self.root;

        loop {
            debug_assert!(current.is_pointer());

            let cons = Cons::from(current);
            let value = self.get(cons.index())?;

            if !value.is_marked() {
                if value.is_pointer() {
                    self.set(cons.index(), previous.mark(true))?;
                    previous = current;
                    current = value;
                } else {
                    self.set(cons.index(), value.mark(true))?;
                }
            } else if cons.index().is_multiple_of(2) {
                current = Cons::new(cons.index() + 1).into();
            } else if !previous.is_pointer() {
                break;
            } else {
                let previous_cons = Cons::from(previous);
                let current_cons = Cons::from(current);
                previous = self.get(previous_cons.index())?;

                self.set(
                    previous_cons.index(),
                    V::from(current_cons.set_index(current_cons.index() - 1)).mark(true),
                )?;

                current = previous_cons.into();
            }
        }

        Ok(())
    }

    fn sweep(&mut self) -> Result<(), Error> {
        self.free = Default::default();

        for index in (0..self.heap().len()).step_by(2) {
            let value = self.get(index)?;

            if value.is_marked() {
                for field in [0, 1] {
                    let index = index + field;
                    self.set(index, self.get(index)?.mark(false))?;
                }
            } else {
                self.set(index + 1, self.free)?;
                self.free = Cons::new(index).into();
            }
        }

        for index in 0..self.heap().len() {
            debug_assert!(!self.get(index)?.is_marked());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value64;
    use core::hash::Hash;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    const HEAP_SIZE: usize = 1 << 10;

    fn assert_free_list<V: Value + Hash, const N: usize>(
        memory: &Memory<V, [V; N]>,
        allocations: usize,
    ) {
        let mut free = memory.free;
        let mut length = 0;

        while free.is_pointer() {
            free = memory.get(Cons::from(free).index() + 1).unwrap();
            length += 1;
        }

        assert_eq!(length, HEAP_SIZE / 2 - allocations);
    }

    fn assert_equal_values<V: Value + Hash, const N: usize>(
        memory: &Memory<V, [V; N]>,
        x: V,
        y: V,
    ) {
        let mut values = Default::default();

        assert_recursive_equal_values(&mut values, memory, x, y)
    }

    fn assert_recursive_equal_values<V: Value + Hash, const N: usize>(
        values: &mut HashSet<V>,
        memory: &Memory<V, [V; N]>,
        x: V,
        y: V,
    ) {
        assert_eq!(x.is_pointer(), y.is_pointer());

        if x.is_pointer() && !values.contains(&x) {
            values.insert(x);

            assert_eq!(x.is_marked(), y.is_marked());

            let x = Cons::from(x);
            let y = Cons::from(y);

            assert_eq!(x.tag(), y.tag());

            for field in [0, 1] {
                assert_recursive_equal_values(
                    values,
                    memory,
                    memory.get(x.index() + field).unwrap(),
                    memory.get(y.index() + field).unwrap(),
                );
            }
        } else {
            assert_eq!(x, y)
        }
    }

    #[test]
    fn create() {
        Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();
    }

    mod allocation {
        use super::*;

        #[test]
        fn allocate_cons_cell() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let x = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let y = memory
                .allocate(Default::default(), Default::default())
                .unwrap();

            assert_equal_values(&memory, x.into(), y.into());
            assert_free_list(&memory, 2);
        }

        #[test]
        fn allocate_two_cons_cells() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let x = memory.allocate(Default::default(), cons.into()).unwrap();

            let cons = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let y = memory.allocate(Default::default(), cons.into()).unwrap();

            assert_equal_values(&memory, x.into(), y.into());
            assert_free_list(&memory, 4);
        }

        #[test]
        fn allocate_three_cons_cells() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let car = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let cdr = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let x = memory.allocate(car.into(), cdr.into()).unwrap();

            let car = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let cdr = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let y = memory.allocate(car.into(), cdr.into()).unwrap();

            assert_equal_values(&memory, x.into(), y.into());
            assert_free_list(&memory, 6);
        }
    }

    mod garbage_collection {
        use super::*;
        use pretty_assertions::assert_eq;

        fn assert_value<V: Value + Hash, const N: usize>(
            memory: &Memory<V, [V; N]>,
            other_memory: &Memory<V, [V; N]>,
            x: V,
        ) {
            let mut values = Default::default();

            assert_recursive_value(&mut values, memory, other_memory, x)
        }

        fn assert_recursive_value<V: Value + Hash, const N: usize>(
            values: &mut HashSet<V>,
            memory: &Memory<V, [V; N]>,
            other_memory: &Memory<V, [V; N]>,
            x: V,
        ) {
            if x.is_pointer() && !values.contains(&x) {
                values.insert(x);

                let x = Cons::from(x);

                for field in [0, 1] {
                    let index = x.index() + field;
                    let value = memory.get(index).unwrap();

                    assert_eq!(value, other_memory.get(index).unwrap());

                    assert_recursive_value(values, memory, other_memory, value);
                }
            }
        }

        #[test]
        fn keep_cons() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory.allocate(1.into(), 2.into()).unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_value(&memory, &old_memory, cons.into());
            assert_free_list(&memory, 1);
        }

        #[test]
        fn keep_two_cons_cells() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory.allocate(1.into(), 2.into()).unwrap();
            let cons = memory.allocate(3.into(), cons.into()).unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_value(&memory, &old_memory, cons.into());
            assert_free_list(&memory, 2);
        }

        #[test]
        fn keep_three_cons_cells() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let car = memory.allocate(1.into(), 2.into()).unwrap();
            let cdr = memory.allocate(3.into(), 4.into()).unwrap();
            let cons = memory.allocate(car.into(), cdr.into()).unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_value(&memory, &old_memory, cons.into());
            assert_free_list(&memory, 3);
        }

        #[test]
        fn keep_recursive_cons_in_car() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory.allocate(Default::default(), 42.into()).unwrap();
            memory.set(cons.index(), cons.into()).unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_value(&memory, &old_memory, cons.into());
            assert_free_list(&memory, 1);
        }

        #[test]
        fn keep_recursive_cons_in_cdr() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory.allocate(42.into(), Default::default()).unwrap();
            memory.set(cons.index() + 1, cons.into()).unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_value(&memory, &old_memory, cons.into());
            assert_free_list(&memory, 1);
        }

        #[test]
        fn collect_recursive_cons_in_car() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory.allocate(Default::default(), 42.into()).unwrap();
            memory.set(cons.index(), cons.into()).unwrap();

            memory.collect_garbages().unwrap();

            assert_free_list(&memory, 0);
        }

        #[test]
        fn collect_recursive_cons_in_cdr() {
            let mut memory =
                Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); _]).unwrap();

            let cons = memory.allocate(42.into(), Default::default()).unwrap();
            memory.set(cons.index() + 1, cons.into()).unwrap();

            memory.collect_garbages().unwrap();

            assert_free_list(&memory, 0);
        }
    }
}
