pub struct NOP {
    opcode: u16,
}

impl NOP {
    pub fn new() -> Self {
        Self {
            opcode: 0b0000000000000000,
        }
    }
}
