use core::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

/// An error of a virtual machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Unexpected end of bytecode.
    BytecodeEnd,
    /// Invalid memory access.
    InvalidMemoryAccess,
    /// Number expected.
    NumberExpected,
    /// Out of memory.
    OutOfMemory,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::BytecodeEnd => write!(formatter, "unexpected end of bytecode"),
            Self::InvalidMemoryAccess => write!(formatter, "invalid memory access"),
            Self::NumberExpected => write!(formatter, "number expected"),
            Self::OutOfMemory => write!(formatter, "out of memory"),
        }
    }
}
