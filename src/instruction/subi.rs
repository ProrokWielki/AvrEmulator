use crate::{instruction::Instruction, registers::Registers};

pub struct SUBI {
    d: u8,
    k: u8,
}

impl Instruction for SUBI {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        let result = registers.r[self.d as usize].wrapping_sub(self.k);

        registers.update_sreg(registers.r[self.d as usize], self.k, result);

        registers.r[self.d as usize] = result as u8;
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
    use crate::{instruction::Instruction, registers::Registers};

    use super::SUBI;

    #[test]
    fn test_process_result_positive() {
        let source_register = 22;
        let source_value: u16 = 27;
        let constant_value: u16 = 18;

        let mut test_registers = Registers::new();
        test_registers.r[source_register as usize] = source_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[source_register as usize] = (source_value - constant_value) as u8;
        expected_registers.pc = 1;

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

        let mut test_registers = Registers::new();
        test_registers.r[source_register as usize] = source_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[source_register as usize] = (source_value - constant_value) as u8;
        expected_registers.pc = 1;
        expected_registers.sreg_z = true;

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

        let mut test_registers = Registers::new();
        test_registers.r[source_register as usize] = source_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[source_register as usize] =
            (source_value.wrapping_sub(constant_value)) as u8;
        expected_registers.pc = 1;
        expected_registers.sreg_c = true;
        expected_registers.sreg_n = true;

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
    fn tests_get_instruction_codes() {
        assert_eq!(SUBI::get_instruction_codes(), vec![0b0101_0000_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(SUBI::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let subi = SUBI::new(0x5123);
        assert_eq!(subi.str(), "subi r18, 19");
    }
}
