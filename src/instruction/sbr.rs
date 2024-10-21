use crate::{instruction::Instruction, registers::Registers};

pub struct SBR {
    d: u16,
    k: u8,
}

impl Instruction for SBR {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        registers.r[self.d as usize] = registers.r[self.d as usize] | self.k
    }
    fn str(&self) -> String {
        return format!("sbr r{}, {}", self.d, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0x6000]
    }
    fn get_instruction_mask() -> u16 {
        0xf000
    }
}

impl SBR {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x00f0) >> 4) + 16,
            k: ((opcode & 0x0f00) >> 4 | (opcode & 0x000f)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::SBR;

    #[test]
    fn test_process_same_register() {
        let k_value = 15;
        let destination_register = 20;
        let register_value = 120;

        let mut test_registers = Registers::new();
        test_registers.r[destination_register as usize] = register_value;

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.r[destination_register as usize] = register_value | k_value;

        let sbr = SBR::new(0x8000 | ((destination_register - 16) << 4) | k_value as u16);
        sbr.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(SBR::get_instruction_codes(), vec![0x6000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(SBR::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let sbr = SBR::new(0x6fff);
        assert_eq!(sbr.str(), "sbr r31, 255");
    }
}
