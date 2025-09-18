//! Moco VM.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod integer;
mod value;

pub use integer::Integer;
pub use value::{Value, Value32, Value64};
