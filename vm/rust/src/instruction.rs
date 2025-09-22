/// An instruction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Creates a cons.
    Cons,
    /// Moves a value.
    Move,
}
