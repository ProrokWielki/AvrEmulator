use crate::{instruction::Instruction, registers::Registers};

pub struct MOVW {
    pc: i32,
    d: u16,
    r: u16,
}

impl Instruction for MOVW {
    fn process(&self, registers: &mut Registers) {
        registers.pc += self.pc;
        registers.set_as_16bit(self.d.into(), registers.as_16bit(self.r as usize).into());
    }
    fn str(&self) -> String {
        return format!("movw r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0000_0001_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1111_0000_0000
    }
}

impl MOVW {
    pub fn new(opcode: u16) -> Self {
        Self {
            pc: 1,
            d: ((opcode & 0x00f0) >> 4) * 2,
            r: (opcode & 0x000f) * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::MOVW;

    #[test]
    fn test_process() {
        let destnation_register: u16 = 7;
        let source_register: u16 = 15;
        let data = 15;

        let mut test_registers = Registers::new();
        test_registers.r[(source_register * 2) as usize] = data;
        test_registers.r[((source_register * 2) + 1) as usize] = data + 1;

        let mut expected_registers = Registers::new();
        expected_registers.r[(source_register * 2) as usize] = data;
        expected_registers.r[((source_register * 2) + 1) as usize] = data + 1;
        expected_registers.r[(destnation_register * 2) as usize] = data;
        expected_registers.r[((destnation_register * 2) + 1) as usize] = data + 1;
        expected_registers.pc = 1;

        let movw = MOVW::new(0xe000 | destnation_register << 4 | source_register);
        movw.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(MOVW::get_instruction_codes(), vec![0b0000_0001_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(MOVW::get_instruction_mask(), 0b1111_1111_0000_0000);
    }

    #[test]
    fn test_str() {
        let movw = MOVW::new(0x014a);
        assert_eq!(movw.str(), "movw r8, r20");
    }
}
