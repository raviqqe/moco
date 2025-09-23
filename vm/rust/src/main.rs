//! The Moco command line tool.

use moco_vm::{Value64, Vm};

const HEAP_SIZE: usize = 1 << 16;

fn main() {
    let vm = Vm::<Value64, [Value64; HEAP_SIZE]>::new([Default::default(); HEAP_SIZE]);

    println!("{:?}", &vm);
}
