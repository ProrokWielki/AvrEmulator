use crate::{instruction::Instruction, memory::Memory};

pub struct POP {
    d: u8,
}

impl Instruction for POP {
    fn process(&self, memory: &mut Memory) {
        memory.set_sp(memory.get_sp() + 1);
        memory.set_register(
            self.d as usize,
            memory.get_stack(memory.get_sp() as usize).unwrap(),
        );
        memory.pc += 1;
    }
    fn str(&self) -> String {
        return format!("pop r{}", self.d).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0000_0000_1111]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1110_0000_1111
    }
}

impl POP {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0b0000_0001_1111_0000) >> 4) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, memory::Memory};

    use super::POP;

    #[test]
    fn test_process() {
        let register = 15;
        let stack_value = 50;
        let start_sp = 10;

        let mut test_registers = Memory::new(500).unwrap();
        test_registers.set_sp(start_sp);
        test_registers.set_stack((start_sp + 1) as usize, stack_value);

        let mut expected_registers = Memory::new(500).unwrap();
        expected_registers.set_sp(start_sp + 1);
        expected_registers.set_register(register as usize, stack_value);
        expected_registers.set_stack((start_sp + 1) as usize, stack_value);
        expected_registers.pc = 1;

        let pop = POP::new(0x900f | register << 4);
        pop.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(POP::get_instruction_codes(), vec![0b1001_0000_0000_1111]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(POP::get_instruction_mask(), 0b1111_1110_0000_1111);
    }

    #[test]
    fn test_str() {
        let pop = POP::new(0x91ff);
        assert_eq!(pop.str(), "pop r31");
    }
}
