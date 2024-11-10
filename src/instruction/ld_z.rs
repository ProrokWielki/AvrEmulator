use crate::{instruction::Instruction, memory::Memory};

pub struct LDZ {
    d: u16,
}

impl Instruction for LDZ {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;

        memory.set_register(
            self.d as usize,
            memory.get_sram(memory.get_z_register() as usize).unwrap(),
        );
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
    use crate::{instruction::Instruction, memory::Memory};

    use super::LDZ;

    #[test]
    fn test_process_register() {
        let z_pointing_address = 20;
        let z_value = 150;
        let destination_register = 15;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(z_pointing_address as usize, z_value);
        test_registers.set_z_register(z_pointing_address);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_register(z_pointing_address as usize, z_value);
        expected_registers.set_z_register(z_pointing_address);
        expected_registers.set_register(
            destination_register as usize,
            expected_registers
                .get_register(z_pointing_address as usize)
                .unwrap(),
        );

        let ldz = LDZ::new(0x8000 | (destination_register << 4) as u16);
        ldz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_io() {
        let z_pointing_address = 64;
        let z_value = 150;
        let destination_register = 15;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_sram(z_pointing_address as usize, z_value);
        test_registers.set_z_register(z_pointing_address);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_sram(z_pointing_address as usize, z_value);
        expected_registers.set_z_register(z_pointing_address);
        expected_registers.set_register(
            destination_register as usize,
            expected_registers
                .get_sram(z_pointing_address as usize)
                .unwrap(),
        );

        let ldz = LDZ::new(0x8000 | (destination_register << 4) as u16);
        ldz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_stack() {
        let z_pointing_address = 256;
        let z_value = 150;
        let destination_register = 15;

        let mut test_registers = Memory::new(500).unwrap();
        test_registers.set_sram(z_pointing_address as usize, z_value);
        test_registers.set_z_register(z_pointing_address);

        let mut expected_registers = Memory::new(500).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_sram(z_pointing_address as usize, z_value);
        expected_registers.set_z_register(z_pointing_address);
        expected_registers.set_register(
            destination_register as usize,
            expected_registers
                .get_sram(z_pointing_address as usize)
                .unwrap(),
        );

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
