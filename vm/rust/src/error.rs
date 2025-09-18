use core::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

/// An error of a virtual machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// A cons expected.
    ConsExpected,
    /// A number expected.
    NumberExpected,
    /// Out of memory.
    OutOfMemory,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ConsExpected => write!(formatter, "cons expected"),
            Self::NumberExpected => write!(formatter, "number expected"),
            Self::OutOfMemory => write!(formatter, "out of memory"),
        }
    }
}
