use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct RJMP {
    k: i16,
}

impl Instruction for RJMP {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc().checked_add_signed(1 + self.k).unwrap());
    }
    fn str(&self) -> String {
        return format!("rjmp {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1100_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_0000_0000_0000
    }
}

impl RJMP {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: (Self::extend((opcode & (!Self::get_instruction_mask())) as i16, 12)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::RJMP;

    #[test]
    fn test_process_positive_k() {
        let mut test_registers = Memory::new(100, vec![]).unwrap();

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_pc(2);

        let nop = RJMP::new(0xf001);
        nop.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_negative_k() {
        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_pc(100);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_pc(100);
        expected_registers.set_pc(expected_registers.get_pc().checked_sub(1).unwrap());

        let nop = RJMP::new(0xcffe);
        nop.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(RJMP::get_instruction_codes(), vec![0b1100_0000_0000_0000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(RJMP::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let rjmp = RJMP::new(0xc001);
        assert_eq!(rjmp.str(), "rjmp 1");

        let rjmp = RJMP::new(0xcfff);
        assert_eq!(rjmp.str(), "rjmp -1");
    }
}
