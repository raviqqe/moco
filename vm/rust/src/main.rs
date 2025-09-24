//! The Moco command line tool.

use core::error::Error;
use moco_vm::{Value64, Vm};

const HEAP_SIZE: usize = 1 << 16;

fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = Vm::<Value64, [Value64; HEAP_SIZE], 0b10>::new([Default::default(); HEAP_SIZE])?;

    vm.run([])?;

    Ok(())
}
