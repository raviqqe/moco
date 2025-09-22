//! Moco VM.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(test)]
extern crate std;

mod cons;
mod error;
mod heap;
mod instruction;
mod integer;
mod memory;
mod value;
mod vm;

pub use cons::Cons;
pub use error::Error;
pub use heap::Heap;
pub use integer::Integer;
pub use memory::Memory;
pub use value::{Value, Value16, Value32, Value64};
pub use vm::Vm;
