use crate::{instruction::Instruction, registers::Registers};

pub struct CPC {
    d: u8,
    r: u8,
}

impl Instruction for CPC {
    fn process(&self, registers: &mut Registers) {
        let result: u8 = (registers.r[self.d as usize])
            .wrapping_sub(registers.r[self.r as usize])
            .wrapping_sub(if registers.sreg_c { 1 } else { 0 });

        registers.pc += 1;

        registers.update_sreg_keep_z_if_resoult_zero(
            registers.r[self.d as usize],
            registers.r[self.r as usize],
            result,
        );
    }
    fn str(&self) -> String {
        return format!("cpc r{}, r{}", self.d, self.r).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0000_0100_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl CPC {
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

    use super::CPC;

    #[test]
    fn test_process_result_positive_wo_carry() {
        let lhs_register = 22;
        let lhs_value = 27;
        let rhs_register = 18;
        let rhs_value = 17;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_positive_with_carry() {
        let lhs_register = 21;
        let lhs_value = 127;
        let rhs_register = 12;
        let rhs_value = 120;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_c = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_positive_zero_before() {
        let lhs_register = 21;
        let lhs_value = 127;
        let rhs_register = 12;
        let rhs_value = 120;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_c = true;
        test_registers.sreg_z = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;
        expected_registers.sreg_z = false;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_wo_carry() {
        let lhs_register = 7;
        let lhs_value = 240;
        let rhs_register = 21;
        let rhs_value = 240;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_wo_carry_zero_before() {
        let lhs_register = 7;
        let lhs_value = 5;
        let rhs_register = 21;
        let rhs_value = 5;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_z = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;
        expected_registers.sreg_z = true;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_with_carry() {
        let lhs_register = 7;
        let lhs_value = 5;
        let rhs_register = 21;
        let rhs_value = 4;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_c = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_with_carry_zero_before() {
        let lhs_register = 7;
        let lhs_value = 5;
        let rhs_register = 21;
        let rhs_value = 4;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_c = true;
        test_registers.sreg_z = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;
        expected_registers.sreg_z = true;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_before_carry_with_carry() {
        let lhs_register = 30;
        let lhs_value = 5;
        let rhs_register = 31;
        let rhs_value = 5;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_c = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;
        expected_registers.sreg_c = true;
        expected_registers.sreg_n = true;
        expected_registers.sreg_h = true;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative_wo_carry() {
        let lhs_register = 1;
        let lhs_value = 100;
        let rhs_register = 2;
        let rhs_value = 200;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;
        expected_registers.sreg_c = true;
        expected_registers.sreg_n = true;
        expected_registers.sreg_h = true;
        expected_registers.sreg_v = true;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative_with_carry() {
        let lhs_register = 0;
        let lhs_value = 1;
        let rhs_register = 31;
        let rhs_value = 15;

        let mut test_registers = Registers::new();
        test_registers.r[lhs_register as usize] = lhs_value;
        test_registers.r[rhs_register as usize] = rhs_value;
        test_registers.sreg_c = true;

        let mut expected_registers = Registers::new();
        expected_registers.r[lhs_register as usize] = lhs_value;
        expected_registers.r[rhs_register as usize] = rhs_value;
        expected_registers.pc = 1;
        expected_registers.sreg_c = true;
        expected_registers.sreg_n = true;
        expected_registers.sreg_h = true;

        let cpc = CPC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        cpc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(CPC::get_instruction_codes(), vec![0b0000_0100_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(CPC::get_instruction_mask(), 0xfc00);
    }

    #[test]
    fn test_str() {
        let sbci = CPC::new(0x0789);
        assert_eq!(sbci.str(), "cpc r24, r25");
    }
}
