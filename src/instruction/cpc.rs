use crate::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct CPC {
    d: u8,
    r: u8,
}

impl Instruction for CPC {
    fn process(&self, memory: &mut Memory) {
        let result: u8 = memory
            .get_register(self.d as usize)
            .unwrap()
            .wrapping_sub(memory.get_register(self.r as usize).unwrap())
            .wrapping_sub(if memory.get_status_register_bit(SregBit::C) {
                1
            } else {
                0
            });

        memory.pc += 1;

        memory.update_sreg_keep_z_if_result_zero(
            memory.get_register(self.d as usize).unwrap(),
            memory.get_register(self.r as usize).unwrap(),
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
    use crate::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::CPC;

    #[test]
    fn test_process_result_positive_wo_carry() {
        let lhs_register = 22;
        let lhs_value = 27;
        let rhs_register = 18;
        let rhs_value = 17;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.clear_status_register_bit(SregBit::Z);

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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::Z);

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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::Z);

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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.set_status_register_bit(SregBit::S);

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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.set_status_register_bit(SregBit::V);

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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.set_status_register_bit(SregBit::S);

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
