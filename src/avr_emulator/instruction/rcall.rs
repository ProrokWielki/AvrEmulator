use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct RCALL {
    k: i32,
}

impl Instruction for RCALL {
    fn process(&self, memory: &mut Memory) {
        memory.set_stack(
            (memory.get_sp() - 1) as usize,
            (((memory.pc + 1) & 0xff00) >> 8) as u8,
        );
        memory.set_stack(memory.get_sp() as usize, ((memory.pc + 1) & 0x00ff) as u8);
        memory.set_sp(memory.get_sp() - 2);
        memory.pc += self.k + 1;
    }
    fn str(&self) -> String {
        return format!("rcall {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1101_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_0000_0000_0000
    }
}

impl RCALL {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: (Self::extend((opcode & (!Self::get_instruction_mask())) as i16, 12) as i32),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::RCALL;

    #[test]
    fn test_process_postivie_k() {
        let k = 13;
        let start_sp = 10;
        let expected_sp = start_sp - 2;

        let mut test_registers = Memory::new(500).unwrap();
        test_registers.set_sp(start_sp);

        let mut expected_registers = Memory::new(500).unwrap();
        expected_registers.set_sp(expected_sp);
        expected_registers.pc = k + 1;

        let rcall = RCALL::new(0xe000 | k as u16);
        rcall.process(&mut test_registers);

        assert_eq!(test_registers.get_sp(), expected_registers.get_sp());
        assert_eq!(test_registers.pc, expected_registers.pc);
    }

    #[test]
    fn test_process_negaitve_k() {
        let k = -2;
        let start_sp = 10;
        let expected_sp = start_sp - 2;

        let mut test_registers = Memory::new(500).unwrap();
        test_registers.set_sp(start_sp);

        let mut expected_registers = Memory::new(500).unwrap();
        expected_registers.set_sp(expected_sp);
        expected_registers.pc = k + 1;

        let rcall = RCALL::new(0xe000 | ((k as i16) & 0x0fff) as u16);
        rcall.process(&mut test_registers);

        assert_eq!(test_registers.get_sp(), expected_registers.get_sp());
        assert_eq!(test_registers.pc, expected_registers.pc);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(RCALL::get_instruction_codes(), vec![0xd000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(RCALL::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let rcall = RCALL::new(0xd003);
        assert_eq!(rcall.str(), "rcall 3");
    }
}
