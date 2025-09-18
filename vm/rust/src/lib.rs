//! Moco VM.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod error;
mod heap;
mod integer;
mod memory;
mod value;

pub use error::Error;
pub use heap::Heap;
pub use integer::Integer;
pub use memory::Memory;
pub use value::{Value, Value32, Value64};
