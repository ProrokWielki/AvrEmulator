use crate::{instruction::Instruction, registers::Registers};

pub struct RET {}

impl Instruction for RET {
    fn process(&self, registers: &mut Registers) {
        registers.set_sp(registers.sp() + 2);
        registers.pc = registers.stack[registers.sp() as usize] as i32;
    }
    fn str(&self) -> String {
        return format!("ret").to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0101_0000_1000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1111_1111_1111
    }
}

impl RET {
    pub fn new(_opcode: u16) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::RET;

    #[test]
    fn test_process() {
        let start_sp = 10;
        let expected_sp = start_sp + 2;
        let expected_pc: i32 = 13;

        let mut test_registers = Registers::new();
        test_registers.set_sp(start_sp);
        test_registers.stack[expected_sp as usize] = expected_pc as u8;

        let mut expected_registers = Registers::new();
        expected_registers.pc = expected_pc;
        expected_registers.set_sp(expected_sp);

        let ret = RET::new(0b1001_0101_0000_1000);
        ret.process(&mut test_registers);

        assert_eq!(test_registers.io, expected_registers.io);
        assert_eq!(test_registers.pc, expected_registers.pc);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(RET::get_instruction_codes(), vec![0b1001_0101_0000_1000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(RET::get_instruction_mask(), 0xffff);
    }

    #[test]
    fn test_str() {
        let ret = RET::new(0b1001_0101_0000_1000);

        assert_eq!(ret.str(), "ret");
    }
}
