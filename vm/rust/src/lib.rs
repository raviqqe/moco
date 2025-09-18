//! Moco VM.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod heap;
mod integer;
mod memory;
mod value;

pub use heap::Heap;
pub use integer::Integer;
pub use memory::Memory;
pub use value::{Value, Value32, Value64};
