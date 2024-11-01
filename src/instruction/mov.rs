use crate::{instruction::Instruction, registers::Registers};

pub struct MOV {
    d: u16,
    r: u16,
}

impl Instruction for MOV {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;
        registers.r[self.d as usize] = registers.r[self.r as usize];
    }
    fn str(&self) -> String {
        return format!("mov r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0010_1100_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl MOV {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x01f0) >> 4),
            r: ((opcode & 0x0200) >> 5) | (opcode & 0x000f),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::MOV;

    #[test]
    fn test_process() {
        let destination_register: u16 = 7;
        let source_register: u16 = 15;
        let data = 15;

        let mut test_registers = Registers::new();
        test_registers.r[(source_register) as usize] = data;

        let mut expected_registers = Registers::new();
        expected_registers.r[(source_register) as usize] = data;
        expected_registers.r[(destination_register) as usize] = data;
        expected_registers.pc = 1;

        let mov = MOV::new(0xe000 | destination_register << 4 | source_register);
        mov.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(MOV::get_instruction_codes(), vec![0b0010_1100_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(MOV::get_instruction_mask(), 0b1111_1100_0000_0000);
    }

    #[test]
    fn test_str() {
        let mov = MOV::new(0x2efe);
        assert_eq!(mov.str(), "mov r15, r30");
    }
}
