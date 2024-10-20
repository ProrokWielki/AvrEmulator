use crate::{instruction::Instruction, registers::Registers};

pub struct STZ {
    d: u16,
}

impl Instruction for STZ {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        registers.stack[registers.z() as usize] = registers.r[self.d as usize];
    }
    fn str(&self) -> String {
        return format!("st z, r{}", self.d).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1000_0010_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0xfe0f
    }
}

impl STZ {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: (opcode & 0b0000_0001_1111_0000) >> 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::STZ;

    #[test]
    fn test_process_same_register() {
        let z_pointing_address = 20;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Registers::new();
        test_registers.r[source_register as usize] = register_value;
        test_registers.set_z(z_pointing_address);

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.stack[z_pointing_address as usize] = register_value;
        expected_registers.r[source_register as usize] = register_value;
        expected_registers.set_z(z_pointing_address);

        let stz = STZ::new(0x8000 | (source_register << 4) as u16);
        stz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(STZ::get_instruction_codes(), vec![0b1000_0010_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(STZ::get_instruction_mask(), 0xfe0f);
    }

    #[test]
    fn test_str() {
        let stz = STZ::new(0x83f0);
        assert_eq!(stz.str(), "st z, r31");
    }
}
