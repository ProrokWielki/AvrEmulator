use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct CPI {
    d: u8,
    k: u8,
}

impl Instruction for CPI {
    fn process(&self, memory: &mut Memory) {
        let result = memory
            .get_register(self.d as usize)
            .unwrap()
            .wrapping_sub(self.k);

        memory.set_pc(memory.get_pc() +1);

        memory.update_sreg(
            memory.get_register(self.d as usize).unwrap(),
            self.k,
            result,
        );
    }
    fn str(&self) -> String {
        return format!("cpi r{}, {}", self.d, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0011_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_0000_0000_0000
    }
}

impl CPI {
    pub fn new(opcode: u16) -> Self {
        let d_value = (opcode & 0b0000_0000_1111_0000) >> 4;
        let k_value = ((opcode & 0b0000_1111_0000_0000) >> 4) | (opcode & 0b0000_0000_0000_1111);

        Self {
            d: (d_value + 16) as u8,
            k: k_value as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::CPI;

    #[test]
    fn test_process_result_positive() {
        let source_register = 22;
        let source_value = 27;
        let constant_value = 18;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, source_value);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_register(source_register as usize, source_value);
        expected_registers.set_pc(1);

        let cpi = CPI::new(
            (0x3000 as u16
                | ((constant_value & 0x00f0) << 4)
                | ((source_register - 16) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        cpi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero() {
        let source_register = 31;
        let source_value = 127;
        let constant_value = 127;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, source_value);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_register(source_register as usize, source_value);
        expected_registers.set_pc(1);
        expected_registers.set_status_register_bit(SregBit::Z);

        let cpi = CPI::new(
            (0x3000 as u16
                | ((constant_value & 0x00f0) << 4)
                | ((source_register - 16) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        cpi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative() {
        let source_register = 16;
        let source_value = 10;
        let constant_value = 100;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(source_register as usize, source_value);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_register(source_register as usize, source_value);
        expected_registers.set_pc(1);
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::S);

        let cpi = CPI::new(
            (0x3000 as u16
                | ((constant_value & 0x00f0) << 4)
                | ((source_register - 16) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        cpi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(CPI::get_instruction_codes(), vec![0b0011_0000_0000_0000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(CPI::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let cpi = CPI::new(0x389a);
        assert_eq!(cpi.str(), "cpi r25, 138");
    }
}
