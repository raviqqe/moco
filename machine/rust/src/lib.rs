//! Moco machine.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(test)]
extern crate std;

mod config;
mod cons;
mod error;
mod heap;
mod instruction;
mod integer;
mod machine;
mod memory;
mod operation_set;
mod value;

pub use cons::Cons;
pub use error::Error;
pub use heap::Heap;
pub use integer::Integer;
pub use machine::Machine;
pub use memory::Memory;
pub use operation_set::{OperationSet, VoidOperationSet};
pub use value::{Value, Value16, Value32, Value64, Value128, ValueSize};
