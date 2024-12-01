use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct StXPlus {
    r: u8,
}

impl Instruction for StXPlus {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() + 1);
        memory.set_sram(
            memory.get_x_register() as usize,
            memory.get_register(self.r as usize).unwrap(),
        );
        memory.set_x_register(memory.get_x_register() + 1);
    }

    fn str(&self) -> String {
        return format!("st x+, r{}", self.r).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0010_0000_1101]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1110_0000_1111
    }
}

impl StXPlus {
    pub fn new(opcode: u16) -> Self {
        Self {
            r: ((opcode & 0b0000_0001_1111_0000) >> 4) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::StXPlus;

    #[test]
    fn test_process_register() {
        let z_pointing_address = 20;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, register_value);
        test_registers.set_x_register(z_pointing_address);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_pc(1);
        expected_registers.set_register(z_pointing_address as usize, register_value);
        expected_registers.set_register(source_register as usize, register_value);
        expected_registers.set_x_register(z_pointing_address + 1);

        let st = StXPlus::new(0x8000 | (source_register << 4) as u16);
        st.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_io() {
        let z_pointing_address = 64;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, register_value);
        test_registers.set_x_register(z_pointing_address);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_pc(1);
        expected_registers.set_io((z_pointing_address - 32) as usize, register_value);
        expected_registers.set_register(source_register as usize, register_value);
        expected_registers.set_x_register(z_pointing_address + 1);

        let st = StXPlus::new(0x8000 | (source_register << 4) as u16);
        st.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_stack() {
        let z_pointing_address = 256;
        let register_value = 150;
        let source_register = 15;

        let mut test_registers = Memory::new(500, vec![]).unwrap();
        test_registers.set_register(source_register as usize, register_value);
        test_registers.set_x_register(z_pointing_address);

        let mut expected_registers = Memory::new(500, vec![]).unwrap();
        expected_registers.set_pc(1);
        expected_registers.set_sram((z_pointing_address) as usize, register_value);
        expected_registers.set_register(source_register as usize, register_value);
        expected_registers.set_x_register(z_pointing_address + 1);

        let st = StXPlus::new(0x8000 | (source_register << 4) as u16);
        st.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(
            StXPlus::get_instruction_codes(),
            vec![0b1001_0010_0000_1101]
        );
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(StXPlus::get_instruction_mask(), 0xfe0f);
    }

    #[test]
    fn test_str() {
        let st = StXPlus::new(0x93fd);
        assert_eq!(st.str(), "st x+, r31");
    }
}
