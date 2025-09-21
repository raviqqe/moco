use crate::{Cons, error::Error, heap::Heap, value::Value};

macro_rules! trace {
    ($scope:literal, $data:expr) => {
        #[cfg(test)]
        std::println!("{}: {:?}", $scope, $data);
    };
}

/// A memory on a virtual machine.
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

        trace!("gc", "begin");

        loop {
            trace!("gc", (previous, current));

            debug_assert!(current.is_pointer() || !self.root.is_pointer());

            let cons = Cons::from(current);
            let value = self.get(cons.index())?;

            if !value.is_marked() {
                trace!("gc", "forward");

                if value.is_pointer() {
                    self.set(cons.index(), previous.mark(true))?;
                    previous = current;
                    current = value;
                } else {
                    self.set(cons.index(), value.mark(true))?;
                }
            } else if cons.index().is_multiple_of(2) {
                trace!("gc", "cdr");
                current = Cons::new(cons.index() + 1).into();
            } else if !previous.is_pointer() {
                trace!("gc", "end");
                break;
            } else {
                trace!("gc", "backward");
                let previous_cons = Cons::from(previous);
                let current_cons = Cons::from(current);
                previous = self.get(previous_cons.index())?;
                self.set(
                    previous_cons.index(),
                    V::from(Cons::new(current_cons.index() - 1).set_tag(current_cons.tag()))
                        .mark(true),
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
                self.set(index, value.mark(false))?;
                self.set(index + 1, self.get(index + 1)?.mark(false))?;
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
    use pretty_assertions::assert_eq;

    const HEAP_SIZE: usize = 1 << 10;

    fn assert_equal_values<const N: usize>(
        x_memory: &Memory<Value64, [Value64; N]>,
        y_memory: &Memory<Value64, [Value64; N]>,
        x: Value64,
        y: Value64,
    ) {
        assert_eq!(x.is_pointer(), y.is_pointer());

        if x.is_pointer() {
            assert_eq!(x.is_marked(), y.is_marked());

            let x = Cons::from(x);
            let y = Cons::from(y);

            assert_eq!(x.tag(), y.tag());

            assert_equal_values(
                x_memory,
                y_memory,
                x_memory.get(x.index()).unwrap(),
                y_memory.get(y.index()).unwrap(),
            );

            assert_equal_values(
                x_memory,
                y_memory,
                x_memory.get(x.index() + 1).unwrap(),
                y_memory.get(y.index() + 1).unwrap(),
            );
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
            let mut memory = Memory::<Value64, [Value64; 2]>::new([Default::default(); _]).unwrap();

            let cons = memory
                .allocate(Default::default(), Default::default())
                .unwrap();

            assert_equal_values(&memory, &memory, cons.into(), cons.into());
        }

        #[test]
        fn allocate_two_cons_cells() {
            let mut memory = Memory::<Value64, [Value64; 8]>::new([Default::default(); _]).unwrap();

            let cons = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let cons = memory.allocate(Default::default(), cons.into()).unwrap();

            assert_equal_values(&memory, &memory, cons.into(), cons.into());
        }

        #[test]
        fn allocate_three_cons_cells() {
            let mut memory = Memory::<Value64, [Value64; 8]>::new([Default::default(); _]).unwrap();

            let car = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let cdr = memory
                .allocate(Default::default(), Default::default())
                .unwrap();
            let cons = memory.allocate(car.into(), cdr.into()).unwrap();

            assert_equal_values(&memory, &memory, cons.into(), cons.into());
        }
    }

    mod garbage_collection {
        use super::*;

        #[test]
        fn collect_cons() {
            let mut memory = Memory::<Value64, [Value64; 2]>::new([Default::default(); _]).unwrap();

            let cons = memory
                .allocate(Value64::from_number(1), Value64::from_number(2))
                .unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_equal_values(&memory, &old_memory, cons.into(), cons.into());
        }

        #[test]
        fn collect_two_cons_cells() {
            let mut memory = Memory::<Value64, [Value64; 8]>::new([Default::default(); _]).unwrap();

            let cons = memory
                .allocate(Value64::from_number(1), Value64::from_number(2))
                .unwrap();
            let cons = memory
                .allocate(Value64::from_number(3), cons.into())
                .unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_equal_values(&memory, &old_memory, cons.into(), cons.into());
        }

        #[test]
        fn collect_three_cons_cells() {
            let mut memory = Memory::<Value64, [Value64; 8]>::new([Default::default(); _]).unwrap();

            let car = memory
                .allocate(Value64::from_number(1), Value64::from_number(2))
                .unwrap();
            let cdr = memory
                .allocate(Value64::from_number(3), Value64::from_number(4))
                .unwrap();
            let cons = memory.allocate(car.into(), cdr.into()).unwrap();
            memory.set_root(cons.into());

            let old_memory = memory.clone();
            memory.collect_garbages().unwrap();

            assert_equal_values(&memory, &old_memory, cons.into(), cons.into());
        }
    }
}
