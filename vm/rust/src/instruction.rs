use crate::cons::Tag;

/// An instruction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Creates a cons.
    Cons,
    /// Moves a value.
    Move,
}

impl Instruction {
    pub(crate) const CONS: Tag = Self::Cons as _;
    pub(crate) const MOVE: Tag = Self::Move as _;
}
