use crate::{instruction::Instruction, registers::Registers};

pub struct PUSH {
    r: u16,
}

impl Instruction for PUSH {
    fn process(&self, regisetrs: &mut Registers) {
        regisetrs.stack[regisetrs.sp() as usize] = (regisetrs.r[self.r as usize]) as u8;
        regisetrs.set_sp(regisetrs.sp() - 1);
        regisetrs.pc += 1
    }
    fn str(&self) -> String {
        return format!("push r{}", self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0010_0000_1111]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1110_0000_1111
    }
}

impl PUSH {
    pub fn new(opcode: u16) -> Self {
        Self {
            r: (opcode & 0b0000_0001_1111_0000) >> 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::PUSH;

    #[test]
    fn test_process() {
        let start_sp = 10;
        let expected_sp = 9;
        let register: u16 = 5;
        let register_value = 15;

        let mut test_registers = Registers::new();
        test_registers.set_sp(start_sp);
        test_registers.r[register as usize] = register_value;

        let mut expected_registers = Registers::new();
        expected_registers.set_sp(expected_sp);
        expected_registers.stack[start_sp as usize] = register_value as u8;
        expected_registers.pc = 1;

        let push = PUSH::new(PUSH::get_instruction_codes()[0] | register << 4);
        push.process(&mut test_registers);

        assert_eq!(test_registers.pc, test_registers.pc);
        assert_eq!(test_registers.sp(), test_registers.sp());
        assert_eq!(test_registers.stack, test_registers.stack);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(PUSH::get_instruction_codes(), vec![0b1001_0010_0000_1111]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(PUSH::get_instruction_mask(), 0b1111_1110_0000_1111);
    }

    #[test]
    fn test_str() {
        let pushed_value: u16 = 5;

        let push: PUSH = PUSH::new(PUSH::get_instruction_codes()[0] | pushed_value << 4);

        assert_eq!(push.str(), "push r5");
    }
}
