/// A heap memory.
pub trait Heap: AsRef<[Value]> + AsMut<[Value]> {}

impl Heap for &mut [Value] {}

impl<const N: usize> Heap for [Value; N] {}

#[cfg(feature = "alloc")]
impl Heap for alloc::vec::Vec<Value> {}
