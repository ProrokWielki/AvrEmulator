use crate::{instruction::Instruction, registers::Registers};

pub struct LDI {
    d: u16,
    k: u16,
}

impl Instruction for LDI {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;
        registers.r[self.d as usize] = self.k as u8
    }
    fn str(&self) -> String {
        return format!("ldi r{}, {}", self.d, self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1110_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_0000_0000_0000
    }
}

impl LDI {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0b0000_0000_1111_0000) >> 4) + 16,
            k: ((opcode & 0b0000_1111_0000_0000) >> 4) | (opcode & 0b0000_0000_0000_1111),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::LDI;

    #[test]
    fn test_process() {
        let destnation_register: u16 = 7;
        let data: u16 = 15;

        let mut test_registers = Registers::new();

        let mut expected_registers = Registers::new();
        expected_registers.r[(destnation_register + 16) as usize] = data as u8;
        expected_registers.pc = 1;

        let ldi = LDI::new(0xe000 | destnation_register << 4 | data);
        ldi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(LDI::get_instruction_codes(), vec![0b1110_0000_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(LDI::get_instruction_mask(), 0b1111_0000_0000_0000);
    }

    #[test]
    fn test_str() {
        let ldi = LDI::new(0xe076);
        assert_eq!(ldi.str(), "ldi r23, 6");
    }
}
