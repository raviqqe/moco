//! Moco VM.

#![no_std]

#[cfg(test)]
extern crate alloc;

mod cons;
mod error;
mod number;
mod value;

pub use self::cons::Cons;
pub use self::error::Error;
pub use self::number::Number;
pub use self::value::Value;
