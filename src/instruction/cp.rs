use crate::{instruction::Instruction, registers::Registers};

pub struct CP {
    d: u8,
    r: u8,
}

impl Instruction for CP {
    fn process(&self, regisetrs: &mut Registers) {
        let result = regisetrs.r[self.d as usize].wrapping_sub(regisetrs.r[self.r as usize]);

        regisetrs.pc += 1;

        regisetrs.update_sreg(
            regisetrs.r[self.d as usize],
            regisetrs.r[self.r as usize],
            result,
        );
    }
    fn str(&self) -> String {
        return format!("cp r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0001_0100_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl CP {
    pub fn new(opcode: u16) -> Self {
        let d_value = (opcode & 0b0000_0001_1111_0000) >> 4;
        let r_value = ((opcode & 0b0000_0010_0000_0000) >> 5) | (opcode & 0b0000_0000_0000_1111);

        Self {
            d: d_value as u8,
            r: r_value as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::CP;

    #[test]
    fn test_process_result_positive() {
        let lhs_register = 21;
        let lhs_value = 10;
        let rhs_register = 22;
        let rhs_value = 5;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value as u8;
        test_registers.r[rhs_register as usize] = rhs_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value as u8;
        expected_registers.r[rhs_register as usize] = rhs_value as u8;
        expected_registers.pc = 1;

        let cp = CP::new(
            (0x1400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cp.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero() {
        let lhs_register = 21;
        let lhs_value = 10;
        let rhs_register = 22;
        let rhs_value = 10;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value as u8;
        test_registers.r[rhs_register as usize] = rhs_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value as u8;
        expected_registers.r[rhs_register as usize] = rhs_value as u8;
        expected_registers.pc = 1;
        expected_registers.sreg_z = true;

        let cp = CP::new(
            (0x1400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cp.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative() {
        let lhs_register = 21;
        let lhs_value = 10;
        let rhs_register = 22;
        let rhs_value = 20;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value as u8;
        test_registers.r[rhs_register as usize] = rhs_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value as u8;
        expected_registers.r[rhs_register as usize] = rhs_value as u8;
        expected_registers.pc = 1;
        expected_registers.sreg_c = true;
        expected_registers.sreg_n = true;

        let cp = CP::new(
            (0x1400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cp.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(CP::get_instruction_codes(), vec![0b0001_0100_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(CP::get_instruction_mask(), 0xfc00);
    }

    #[test]
    fn test_str() {
        let cp = CP::new(0x4723);
        assert_eq!(cp.str(), "cp r18, r19");
    }
}
