use crate::{instruction::Instruction, memory::Memory};

pub struct RJMP {
    k: i32,
}

impl Instruction for RJMP {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1 + self.k;
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
            k: (Self::extend((opcode & (!Self::get_instruction_mask())) as i16, 12) as i32),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, memory::Memory};

    use super::RJMP;

    #[test]
    fn test_process_positive_k() {
        let mut test_registers = Memory::new(100).unwrap();

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 2;

        let nop = RJMP::new(0xf001);
        nop.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_negative_k() {
        let mut test_registers = Memory::new(100).unwrap();

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = -1;

        let nop = RJMP::new(0xcffe);
        nop.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(RJMP::get_instruction_codes(), vec![0b1100_0000_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
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
