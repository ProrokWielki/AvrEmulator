use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct STZ {
    d: u16,
}

impl Instruction for STZ {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;
        memory.set_sram(
            memory.get_z_register() as usize,
            memory.get_register(self.d as usize).unwrap(),
        )
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
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::STZ;

    #[test]
    fn test_process_register() {
        let z_pointing_address = 20;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(source_register as usize, register_value);
        test_registers.set_z_register(z_pointing_address);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_register(z_pointing_address as usize, register_value);
        expected_registers.set_register(source_register as usize, register_value);
        expected_registers.set_z_register(z_pointing_address);

        let stz = STZ::new(0x8000 | (source_register << 4) as u16);
        stz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_io() {
        let z_pointing_address = 64;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(source_register as usize, register_value);
        test_registers.set_z_register(z_pointing_address);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_io((z_pointing_address - 32) as usize, register_value);
        expected_registers.set_register(source_register as usize, register_value);
        expected_registers.set_z_register(z_pointing_address);

        let stz = STZ::new(0x8000 | (source_register << 4) as u16);
        stz.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_stack() {
        let z_pointing_address = 256;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Memory::new(500).unwrap();
        test_registers.set_register(source_register as usize, register_value);
        test_registers.set_z_register(z_pointing_address);

        let mut expected_registers = Memory::new(500).unwrap();
        expected_registers.pc = 1;
        expected_registers.set_sram((z_pointing_address) as usize, register_value);
        expected_registers.set_register(source_register as usize, register_value);
        expected_registers.set_z_register(z_pointing_address);

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
