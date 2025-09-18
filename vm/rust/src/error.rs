use core::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

/// An error of a virtual machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Invalid memory access.
    InvalidMemoryAccess,
    /// Out of memory.
    OutOfMemory,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidMemoryAccess => write!(formatter, "invalid memory access"),
            Self::OutOfMemory => write!(formatter, "out of memory"),
        }
    }
}
