//! The Moco command line tool.

use moco_vm::{Memory, Value64};

const HEAP_SIZE: usize = 1 << 16;

fn main() {
    let memory = Memory::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); HEAP_SIZE]);

    println!("{:?}", &memory);
}
