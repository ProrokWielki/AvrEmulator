use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct SUBI {
    d: u8,
    k: u8,
}

impl Instruction for SUBI {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() + 1);

        let result = memory
            .get_register(self.d as usize)
            .unwrap()
            .wrapping_sub(self.k);

        memory.update_sreg(
            memory.get_register(self.d as usize).unwrap(),
            self.k,
            result,
        );

        memory.set_register(self.d as usize, result as u8);
    }
    fn str(&self) -> String {
        return format!("subi r{}, {}", self.d, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0101_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_0000_0000_0000
    }
}

impl SUBI {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: (((opcode & 0x00f0) >> 4) + 16) as u8,
            k: (((opcode & 0x0f00) >> 4) | (opcode & 0x000f)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::SUBI;

    #[test]
    fn test_process_result_positive() {
        let source_register = 22;
        let source_value: u16 = 27;
        let constant_value: u16 = 18;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, source_value as u8);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_register(
            source_register as usize,
            (source_value - constant_value) as u8,
        );
        expected_registers.set_pc(1);

        let subi = SUBI::new(
            (0x5000 as u16
                | ((constant_value & 0x00f0) << 4)
                | ((source_register - 16) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        subi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero() {
        let source_register = 27;
        let source_value: u16 = 30;
        let constant_value: u16 = 30;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, source_value as u8);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_register(
            source_register as usize,
            (source_value - constant_value) as u8,
        );
        expected_registers.set_pc(1);
        expected_registers.set_status_register_bit(SregBit::Z);

        let subi = SUBI::new(
            (0x5000 as u16
                | ((constant_value & 0x00f0) << 4)
                | ((source_register - 16) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        subi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative() {
        let source_register = 21;
        let source_value: u16 = 10;
        let constant_value: u16 = 20;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, source_value as u8);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_register(
            source_register as usize,
            (source_value.wrapping_sub(constant_value)) as u8,
        );
        expected_registers.set_pc(1);
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::S);

        let subi = SUBI::new(
            (0x5000 as u16
                | ((constant_value & 0x00f0) << 4)
                | ((source_register - 16) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        subi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(SUBI::get_instruction_codes(), vec![0b0101_0000_0000_0000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(SUBI::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let subi = SUBI::new(0x5123);
        assert_eq!(subi.str(), "subi r18, 19");
    }
}
