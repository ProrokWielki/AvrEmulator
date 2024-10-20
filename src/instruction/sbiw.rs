use crate::{instruction::Instruction, registers::Registers};

pub struct SBIW {
    d: u8,
    k: u16,
}

impl Instruction for SBIW {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        let result = registers.as_16bit(self.d as usize).wrapping_sub(self.k);
        let rd = registers.as_16bit(self.d as usize);

        registers.update_sreg_16bit(rd, self.k, result);

        registers.set_as_16bit(self.d as usize, result);
    }
    fn str(&self) -> String {
        return format!("sbiw r{}:r{}, {}", self.d + 1, self.d, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0111_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1111_0000_0000
    }
}

impl SBIW {
    const POSSIBLE_D: [u8; 4] = [24, 26, 28, 30];

    pub fn new(opcode: u16) -> Self {
        let d_value: u16 = (opcode & 0b0000_0000_0011_0000) >> 4;
        Self {
            d: Self::POSSIBLE_D[d_value as usize],
            k: (((opcode & 0b0000_0000_1100_0000) >> 2) | (opcode & 0x000f)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::SBIW;

    #[test]
    fn test_process_result_positive() {
        let source_register = 0;
        let source_value = 400;
        let constant_value = 10;

        let mut test_registers = Registers::new();
        test_registers.set_as_16bit(
            SBIW::POSSIBLE_D[source_register as usize] as usize,
            source_value,
        );

        let mut expected_registers = Registers::new();
        expected_registers.set_as_16bit(
            SBIW::POSSIBLE_D[source_register as usize] as usize,
            source_value - constant_value,
        );
        expected_registers.pc = 1;

        let sbiw = SBIW::new(
            (0x9700 as u16
                | ((constant_value & 0x0030) << 2)
                | ((source_register) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        sbiw.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero() {
        let source_register = 2;
        let source_value = 63;
        let constant_value = 63;

        let mut test_registers = Registers::new();
        test_registers.set_as_16bit(
            SBIW::POSSIBLE_D[source_register as usize] as usize,
            source_value,
        );

        let mut expected_registers = Registers::new();
        expected_registers.set_as_16bit(
            SBIW::POSSIBLE_D[source_register as usize] as usize,
            source_value - constant_value,
        );
        expected_registers.pc = 1;
        expected_registers.sreg_z = true;

        let sbiw = SBIW::new(
            (0x9700 as u16
                | ((constant_value & 0x0030) << 2)
                | ((source_register) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        sbiw.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative() {
        let source_register = 2;
        let source_value = 5;
        let constant_value = 10;

        let mut test_registers = Registers::new();
        test_registers.set_as_16bit(
            SBIW::POSSIBLE_D[source_register as usize] as usize,
            source_value,
        );

        let mut expected_registers = Registers::new();
        expected_registers.set_as_16bit(
            SBIW::POSSIBLE_D[source_register as usize] as usize,
            source_value.wrapping_sub(constant_value),
        );
        expected_registers.pc = 1;
        expected_registers.sreg_c = true;
        expected_registers.sreg_v = true;
        expected_registers.sreg_n = true;

        let sbiw = SBIW::new(
            (0x9700 as u16
                | ((constant_value & 0x0030) << 2)
                | ((source_register) << 4)
                | (constant_value & 0x000f)) as u16,
        );
        sbiw.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(SBIW::get_instruction_codes(), vec![0b1001_0111_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(SBIW::get_instruction_mask(), 0xff00);
    }

    #[test]
    fn test_str() {
        let sbiw = SBIW::new(0x97ff);
        assert_eq!(sbiw.str(), "sbiw r31:r30, 63");
    }
}
