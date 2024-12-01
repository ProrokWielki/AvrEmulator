use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct BRBS {
    s: u8,
    k: i16,
}

impl Instruction for BRBS {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() + 1);

        let mut bit_set = false;

        match self.s {
            0 => bit_set = memory.get_status_register_bit(SregBit::C),
            1 => bit_set = memory.get_status_register_bit(SregBit::Z),
            2 => bit_set = memory.get_status_register_bit(SregBit::N),
            3 => bit_set = memory.get_status_register_bit(SregBit::V),
            4 => bit_set = memory.get_status_register_bit(SregBit::S),
            5 => bit_set = memory.get_status_register_bit(SregBit::H),
            6 => bit_set = memory.get_status_register_bit(SregBit::T),
            7 => bit_set = memory.get_status_register_bit(SregBit::I),
            _ => (),
        }

        if bit_set {
            memory.set_pc(memory.get_pc().checked_add_signed(self.k).unwrap());
        }
    }
    fn str(&self) -> String {
        return format!("brbs {}, {}", self.s, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0xf000]
    }
    fn get_instruction_mask() -> u16 {
        0xfc00
    }
}

impl BRBS {
    pub fn new(opcode: u16) -> Self {
        Self {
            s: (opcode & 0x0007) as u8,
            k: Self::extend(((opcode & 0x03f8) >> 3) as i16, 7),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::BRBS;

    #[test]
    fn test_process() {
        let sreg_bit = 5;
        let k = 15;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_status_register_bit(SregBit::H);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_pc(1 + k);
        expected_registers.set_status_register_bit(SregBit::H);

        let brbs = BRBS::new((0xf000 | (k << 3) | (sreg_bit)) as u16);
        brbs.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(BRBS::get_instruction_codes(), vec![0xf000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(BRBS::get_instruction_mask(), 0xfc00);
    }

    #[test]
    fn test_str() {
        let brbs = BRBS::new(0xf3fd);
        assert_eq!(brbs.str(), "brbs 5, -1");
    }
}
