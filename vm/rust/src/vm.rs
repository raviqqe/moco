pub struct Vm<H: Heap> {
    memory: Memory,
}

impl<H: Heap> Vm {
    pub fn new() -> Self {
        Self {}
    }
}
