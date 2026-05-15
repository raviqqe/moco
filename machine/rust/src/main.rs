//! The Moco command line tool.

use core::error::Error;
use moco_machine::{Value64, Machine, VoidOperationSet};

const HEAP_SIZE: usize = 1 << 16;

fn main() -> Result<(), Box<dyn Error>> {
    let mut machine = Machine::<Value64, [Value64; HEAP_SIZE], 0b10, _>::new(
        [Default::default(); HEAP_SIZE],
        VoidOperationSet::new(),
    )?;

    machine.run([])?;

    Ok(())
}
