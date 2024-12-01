use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct BRLT {
    k: i16,
}

impl Instruction for BRLT {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() + 1);

        if memory.get_status_register_bit(SregBit::N) != memory.get_status_register_bit(SregBit::V)
        {
            memory.set_pc(memory.get_pc().checked_add_signed(self.k).unwrap());
        }
    }
    fn str(&self) -> String {
        return format!("brlt {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1111_0000_0000_0100]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0111
    }
}

impl BRLT {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: Self::extend(((opcode & 0b0000_0011_1111_1000) >> 3) as i16, 7),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::BRLT;

    #[test]
    fn test_process_true() {
        let k = 7;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_status_register_bit(SregBit::N);
        test_registers.clear_status_register_bit(SregBit::V);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.clear_status_register_bit(SregBit::V);
        expected_registers.set_pc(1 + k);

        let brlt = BRLT::new(0xf004 | (k << 3) as u16);
        brlt.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_false() {
        let k = 7;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_status_register_bit(SregBit::N);
        test_registers.set_status_register_bit(SregBit::V);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::V);
        expected_registers.set_pc(1);

        let brlt = BRLT::new(0xf004 | (k << 3) as u16);
        brlt.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(BRLT::get_instruction_codes(), vec![0b1111_0000_0000_0100]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(BRLT::get_instruction_mask(), 0b1111_1100_0000_0111);
    }

    #[test]
    fn test_str() {
        let brlt = BRLT::new(0xf3fc);
        assert_eq!(brlt.str(), "brlt -1");
    }
}
