use crate::{Heap, value::Value};

const CONS_FIELD_COUNT: usize = 2;

/// A memory on a virtual machine.
pub struct Memory<V, H: Heap<T>> {
    root: V,
    heap: H,
}

impl<T: Heap> Memory<T> {
    /// Creates a memory.
    pub fn new(heap: &'a mut [Value]) -> Result<Self, Error> {
        let mut memory = Self {
            code: NEVER,
            stack: NEVER,
            r#false: NEVER,
            register: NEVER,
            allocation_index: 0,
            space: false,
            heap,
        };

        // Initialize a fake false value.
        let cons = memory.allocate_unchecked(Default::default(), Default::default())?;
        memory.r#false = memory.allocate_unchecked(cons.into(), cons.into())?;

        Ok(memory)
    }

    /// Returns a code.
    #[inline]
    pub const fn code(&self) -> Cons {
        self.code
    }

    /// Sets a code.
    #[inline]
    pub const fn set_code(&mut self, value: Cons) {
        self.code = value;
    }

    /// Returns a register.
    #[inline]
    pub const fn register(&self) -> Cons {
        self.register
    }

    /// Sets a register.
    #[inline]
    pub const fn set_register(&mut self, value: Cons) {
        self.register = value;
    }

    /// Returns a stack.
    #[inline]
    pub const fn stack(&self) -> Cons {
        self.stack
    }

    /// Sets a stack.
    #[inline]
    pub const fn set_stack(&mut self, value: Cons) {
        self.stack = value;
    }

    /// Returns a boolean value.
    #[inline]
    pub fn boolean(&self, value: bool) -> Result<Cons, Error> {
        Ok(if value {
            self.cdr(self.r#false)?.assume_cons()
        } else {
            self.r#false
        })
    }

    /// Returns a null value.
    #[inline]
    pub fn null(&self) -> Result<Cons, Error> {
        Ok(self.car(self.r#false)?.assume_cons())
    }

    /// Sets a false value.
    #[inline]
    pub(crate) const fn set_false(&mut self, cons: Cons) {
        self.r#false = cons;
    }

    /// Pushes a value to a stack.
    #[inline(always)]
    pub fn push(&mut self, value: Value) -> Result<(), Error> {
        self.stack = self.cons(value, self.stack)?;

        Ok(())
    }

    /// Pops a value from a stack.
    #[inline]
    pub fn pop(&mut self) -> Result<Value, Error> {
        debug_assert_ne!(self.stack, self.null()?);

        let value = self.car(self.stack)?;
        self.stack = self.cdr(self.stack)?.assume_cons();
        Ok(value)
    }

    /// Pops values from a stack.
    pub fn pop_many<const M: usize>(&mut self) -> Result<[Value; M], Error> {
        let mut values = [Default::default(); M];

        for index in 0..=M - 1 {
            values[M - 1 - index] = self.pop()?;
        }

        Ok(values)
    }

    /// Pops numbers from a stack.
    pub fn pop_numbers<const M: usize>(&mut self) -> Result<[Number; M], Error> {
        let mut numbers = [Default::default(); M];

        for (index, value) in self.pop_many::<M>()?.into_iter().enumerate() {
            numbers[index] = value.assume_number();
        }

        Ok(numbers)
    }

    /// Peeks a value at the top of a stack.
    #[inline]
    pub fn top(&self) -> Result<Value, Error> {
        debug_assert_ne!(self.stack, self.null()?);

        self.car(self.stack)
    }

    /// Sets a value at the top of a stack.
    #[inline]
    pub fn set_top(&mut self, value: Value) -> Result<(), Error> {
        self.set_car(self.stack, value)
    }

    /// Allocates a cons with a default tag of [`Type::Pair`].
    #[inline]
    pub fn cons(&mut self, car: Value, cdr: Cons) -> Result<Cons, Error> {
        self.allocate(car, cdr.set_tag(Type::Pair as Tag).into())
    }

    /// Allocates a cons.
    #[inline]
    pub fn allocate(&mut self, car: Value, cdr: Value) -> Result<Cons, Error> {
        let mut cons = self.allocate_unchecked(car, cdr)?;

        debug_assert_eq!(cons.tag(), Type::default() as Tag);
        assert_heap_cons!(self, cons);
        assert_heap_value!(self, car);
        assert_heap_value!(self, cdr);

        if self.is_out_of_memory() || cfg!(feature = "gc_always") {
            self.collect_garbages(Some(&mut cons))?;
        }

        Ok(cons)
    }

    #[inline]
    fn allocate_unchecked(&mut self, car: Value, cdr: Value) -> Result<Cons, Error> {
        if self.is_out_of_memory() {
            return Err(Error::OutOfMemory);
        }

        let cons = Cons::new(self.allocation_end() as u64);
        self.allocation_index += CONS_FIELD_COUNT;

        assert_heap_cons!(self, cons);

        self.set_car(cons, car)?;
        self.set_raw_cdr(cons, cdr)?;

        debug_assert!(self.allocation_index <= self.space_size());

        Ok(cons)
    }

    #[inline]
    const fn is_out_of_memory(&self) -> bool {
        self.allocation_index >= self.space_size()
    }

    /// Returns a heap size.
    #[inline]
    pub const fn size(&self) -> usize {
        self.heap.len()
    }

    #[inline]
    const fn space_size(&self) -> usize {
        self.size() / 2
    }

    /// Returns the current allocation index relative an allocation start index.
    #[inline]
    pub const fn allocation_index(&self) -> usize {
        self.allocation_index
    }

    /// Returns an allocation start index.
    #[inline]
    pub const fn allocation_start(&self) -> usize {
        if self.space { self.space_size() } else { 0 }
    }

    /// Returns an allocation end index.
    #[inline]
    pub const fn allocation_end(&self) -> usize {
        self.allocation_start() + self.allocation_index
    }

    #[inline]
    fn get<const G: bool>(&self, index: usize) -> Result<Value, Error> {
        assert_heap_index!(self, index, G);

        self.heap
            .get(index)
            .copied()
            .ok_or(Error::InvalidMemoryAccess)
    }

    #[inline]
    fn set<const G: bool>(&mut self, index: usize, value: Value) -> Result<(), Error> {
        assert_heap_index!(self, index, G);

        *self.heap.get_mut(index).ok_or(Error::InvalidMemoryAccess)? = value;

        Ok(())
    }

    /// Returns a value of a `car` field in a cons.
    #[inline]
    pub fn car(&self, cons: Cons) -> Result<Value, Error> {
        self.get::<false>(cons.index())
    }

    /// Returns a value of a `cdr` field in a cons.
    #[inline]
    pub fn cdr(&self, cons: Cons) -> Result<Value, Error> {
        self.get::<false>(cons.index() + 1)
    }

    #[inline]
    fn garbage_car(&self, cons: Cons) -> Result<Value, Error> {
        self.get::<true>(cons.index())
    }

    #[inline]
    fn garbage_cdr(&self, cons: Cons) -> Result<Value, Error> {
        self.get::<true>(cons.index() + 1)
    }

    /// Returns a value of a `car` field in a value assumed as a cons.
    #[inline]
    pub fn car_value(&self, cons: Value) -> Result<Value, Error> {
        self.car(cons.assume_cons())
    }

    /// Returns a value of a `cdr` field in a value assumed as a cons.
    #[inline]
    pub fn cdr_value(&self, cons: Value) -> Result<Value, Error> {
        self.cdr(cons.assume_cons())
    }

    #[inline]
    fn set_field<const G: bool>(
        &mut self,
        cons: Cons,
        index: usize,
        value: Value,
    ) -> Result<(), Error> {
        self.set::<G>(cons.index() + index, value)
    }

    /// Sets a value to a `car` field in a cons.
    #[inline]
    pub fn set_car(&mut self, cons: Cons, value: Value) -> Result<(), Error> {
        self.set_field::<false>(cons, 0, value)
    }

    /// Sets a value to a `cdr` field in a cons.
    #[inline]
    pub fn set_cdr(&mut self, cons: Cons, value: Value) -> Result<(), Error> {
        // Keep an existing tag.
        self.set_field::<false>(
            cons,
            1,
            value.set_tag(self.get::<false>(cons.index() + 1)?.tag()),
        )
    }

    /// Sets a raw value to a `cdr` field in a cons overwriting its tag.
    #[inline]
    pub fn set_raw_cdr(&mut self, cons: Cons, value: Value) -> Result<(), Error> {
        self.set_field::<false>(cons, 1, value)
    }

    #[inline]
    fn set_garbage_car(&mut self, cons: Cons, value: Value) -> Result<(), Error> {
        self.set_field::<true>(cons, 0, value)
    }

    #[inline]
    fn set_garbage_cdr(&mut self, cons: Cons, value: Value) -> Result<(), Error> {
        self.set_field::<true>(cons, 1, value)
    }

    /// Sets a value to a `car` field in a value assumed as a cons.
    #[inline(always)]
    pub fn set_car_value(&mut self, cons: Value, value: Value) -> Result<(), Error> {
        self.set_car(cons.assume_cons(), value)
    }

    /// Sets a value to a `cdr` field in a value assumed as a cons.
    #[inline(always)]
    pub fn set_cdr_value(&mut self, cons: Value, value: Value) -> Result<(), Error> {
        self.set_cdr(cons.assume_cons(), value)
    }

    /// Returns a tail of a list.
    #[inline(always)]
    pub fn tail(&self, mut list: Cons, mut index: usize) -> Result<Cons, Error> {
        while index > 0 {
            list = self.cdr(list)?.assume_cons();
            index -= 1;
        }

        Ok(list)
    }

    /// Builds a string.
    pub fn build_string(&mut self, string: &str) -> Result<Cons, Error> {
        let string = self.build_raw_string(string)?;
        let length = Number::from_i64(self.list_length(string)? as _).into();
        self.allocate(length, string.set_tag(Type::String as _).into())
    }

    /// Builds a raw string.
    pub fn build_raw_string(&mut self, string: &str) -> Result<Cons, Error> {
        let mut list = self.null()?;
        self.build_intermediate_string(string, &mut list)?;
        Ok(list)
    }

    fn build_intermediate_string(&mut self, string: &str, list: &mut Cons) -> Result<(), Error> {
        for character in string.chars().rev() {
            *list = self.cons(Number::from_i64(character as _).into(), *list)?;
        }

        Ok(())
    }

    /// Executes an operation against a value at the top of a stack.
    pub fn operate_top(
        &mut self,
        operate: impl Fn(&Self, Value) -> Result<Value, Error>,
    ) -> Result<(), Error> {
        let value = self.pop()?;
        self.push(operate(self, value)?)?;
        Ok(())
    }

    /// Calculates a length of a list.
    pub fn list_length(&self, mut list: Cons) -> Result<usize, Error> {
        let mut length = 0;

        while list != self.null()? {
            length += 1;
            list = self.cdr(list)?.assume_cons();
        }

        Ok(length)
    }

    /// Executes an unary number operation.
    pub fn operate_unary(&mut self, operate: impl Fn(Number) -> Number) -> Result<(), Error> {
        let [x] = self.pop_numbers()?;

        self.push(operate(x).into())?;

        Ok(())
    }

    /// Executes a binary number operation.
    pub fn operate_binary(&mut self, operate: fn(Number, Number) -> Number) -> Result<(), Error> {
        let [x, y] = self.pop_numbers()?;

        self.push(operate(x, y).into())?;

        Ok(())
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
