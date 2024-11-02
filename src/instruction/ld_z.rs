use crate::{instruction::Instruction, registers::Registers};

pub struct LDZ {
    d: u16,
}

impl Instruction for LDZ {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        if registers.z() < 32 {
            registers.r[self.d as usize] = registers.r[registers.z() as usize];
        } else if registers.z() < 32 + 64 {
            registers.r[self.d as usize] = registers.io[(registers.z() - 32) as usize];
        } else {
            registers.r[self.d as usize] = registers.stack[(registers.z() - (32 + 64)) as usize];
        }
    }

    fn str(&self) -> String {
        return format!("ld r{}, z", self.d).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1000_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0xfe0f
    }
}

impl LDZ {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: (opcode & 0b0000_0001_1111_0000) >> 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::LDZ;

    #[test]
    fn test_process_register() {
        let z_pointing_address = 20;
        let z_value = 150;
        let destination_register = 15;

        let mut test_registers = Registers::new();
        test_registers.r[z_pointing_address as usize] = z_value;
        test_registers.set_z(z_pointing_address);

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.r[z_pointing_address as usize] = z_value;
        expected_registers.set_z(z_pointing_address);
        expected_registers.r[destination_register as usize] =
            expected_registers.r[z_pointing_address as usize];

        let ldz = LDZ::new(0x8000 | (destination_register << 4) as u16);
        ldz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_io() {
        let z_pointing_address = 64;
        let z_value = 150;
        let destination_register = 15;

        let mut test_registers = Registers::new();
        test_registers.io[(z_pointing_address - 32) as usize] = z_value;
        test_registers.set_z(z_pointing_address);

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.io[(z_pointing_address - 32) as usize] = z_value;
        expected_registers.set_z(z_pointing_address);
        expected_registers.r[destination_register as usize] =
            expected_registers.io[(z_pointing_address - 32) as usize];

        let ldz = LDZ::new(0x8000 | (destination_register << 4) as u16);
        ldz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_stack() {
        let z_pointing_address = 256;
        let z_value = 150;
        let destination_register = 15;

        let mut test_registers = Registers::new();
        test_registers.stack[(z_pointing_address - (32 + 64)) as usize] = z_value;
        test_registers.set_z(z_pointing_address);

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.stack[(z_pointing_address - (32 + 64)) as usize] = z_value;
        expected_registers.set_z(z_pointing_address);
        expected_registers.r[destination_register as usize] =
            expected_registers.stack[(z_pointing_address - (32 + 64)) as usize];

        let ldz = LDZ::new(0x8000 | (destination_register << 4) as u16);
        ldz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(LDZ::get_instruction_codes(), vec![0b1000_0000_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(LDZ::get_instruction_mask(), 0xfe0f);
    }

    #[test]
    fn test_str() {
        let ldz = LDZ::new(0x81f0);
        assert_eq!(ldz.str(), "ld r31, z");
    }
}
