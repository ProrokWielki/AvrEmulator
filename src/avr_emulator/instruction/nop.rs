use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct NOP {}

impl Instruction for NOP {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;
    }
    fn str(&self) -> String {
        return "nop".to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0000_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1111_1111_1111
    }
}

impl NOP {
    pub fn new(_opcode: u16) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::NOP;

    #[test]
    fn test_process() {
        let mut test_registers = Memory::new(100).unwrap();

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 1;

        let nop = NOP::new(0x0000);
        nop.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(NOP::get_instruction_codes(), vec![0x0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(NOP::get_instruction_mask(), 0xffff);
    }

    #[test]
    fn test_str() {
        let nop = NOP::new(0x0000);

        assert_eq!(nop.str(), "nop");
    }
}
