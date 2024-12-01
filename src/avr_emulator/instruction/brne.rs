use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct BRNE {
    k: i32,
}

impl Instruction for BRNE {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;

        if memory.get_status_register_bit(SregBit::Z) == false {
            memory.pc += self.k;
        }
    }
    fn str(&self) -> String {
        return format!("brne {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1111_0100_0000_0001]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0111
    }
}

impl BRNE {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: Self::extend(((opcode & 0b0000_0011_1111_1000) >> 3) as i16, 7) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::BRNE;

    #[test]
    fn test_process_true() {
        let k = 7;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.clear_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.clear_status_register_bit(SregBit::Z);
        expected_registers.pc = 1 + k;

        let brne = BRNE::new(0xf001 | (k << 3) as u16);
        brne.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_false() {
        let k = 7;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_status_register_bit(SregBit::Z);
        expected_registers.pc = 1;

        let brne = BRNE::new(0xf001 | (k << 3) as u16);
        brne.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(BRNE::get_instruction_codes(), vec![0b1111_0100_0000_0001]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(BRNE::get_instruction_mask(), 0b1111_1100_0000_0111);
    }

    #[test]
    fn test_str() {
        let brne = BRNE::new(0xf7f9);
        assert_eq!(brne.str(), "brne -1");
    }
}
