/// A heap memory.
pub trait Heap<T>: AsRef<[T]> + AsMut<[T]> {}

impl<T> Heap<T> for &mut [T] {}

impl<T, const N: usize> Heap<T> for [T; N] {}

#[cfg(feature = "alloc")]
impl Heap<T> for alloc::vec::Vec<T> {}
