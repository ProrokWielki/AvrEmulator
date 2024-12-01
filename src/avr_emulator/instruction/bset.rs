use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct BSET {
    s: u8,
}

impl Instruction for BSET {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;

        match self.s {
            0 => memory.set_status_register_bit(SregBit::C),
            1 => memory.set_status_register_bit(SregBit::Z),
            2 => memory.set_status_register_bit(SregBit::N),
            3 => memory.set_status_register_bit(SregBit::V),
            4 => memory.set_status_register_bit(SregBit::S),
            5 => memory.set_status_register_bit(SregBit::H),
            6 => memory.set_status_register_bit(SregBit::T),
            7 => memory.set_status_register_bit(SregBit::I),
            _ => (),
        }
    }
    fn str(&self) -> String {
        return format!("bset {}", self.s).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0x9408]
    }
    fn get_instruction_mask() -> u16 {
        0xff8f
    }
}

impl BSET {
    pub fn new(opcode: u16) -> Self {
        Self {
            s: ((opcode & 0x0070) >> 4) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::BSET;

    #[test]
    fn test_process() {
        let sreg_bit = 5;

        let mut test_registers = Memory::new(100).unwrap();

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::H);

        let bset = BSET::new(0x9408 | (sreg_bit << 4) as u16);
        bset.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(BSET::get_instruction_codes(), vec![0x9408]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(BSET::get_instruction_mask(), 0xff8f);
    }

    #[test]
    fn test_str() {
        let bset = BSET::new(0x9478);
        assert_eq!(bset.str(), "bset 7");
    }
}
