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
    pub(crate) const CONS: Tag = Instruction::Cons as _;
    pub(crate) const MOVE: Tag = Instruction::Move as _;
}
