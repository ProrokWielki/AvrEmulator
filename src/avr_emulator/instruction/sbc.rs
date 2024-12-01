use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct SBC {
    d: u8,
    r: u8,
}

impl Instruction for SBC {
    fn process(&self, memory: &mut Memory) {
        let result = memory
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

        memory.set_register(self.d as usize, result);
    }
    fn str(&self) -> String {
        return format!("sbc r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0000_1000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl SBC {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x01f0) >> 4) as u8,
            r: (((opcode & 0b0000_0010_0000_0000) >> 5) | (opcode & 0x000f)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::SBC;

    #[test]
    fn test_process_result_positive_wo_carry() {
        let lhs_register = 22;
        let lhs_value = 27;
        let rhs_register = 30;
        let rhs_value = 18;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, (lhs_value - rhs_value) as u8);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_positive_with_carry() {
        let lhs_register = 31;
        let lhs_value = 255;
        let rhs_register = 30;
        let rhs_value = 250;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, (lhs_value - rhs_value - 1) as u8);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

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
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value - rhs_value);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.clear_status_register_bit(SregBit::Z);

        let sbc = SBC::new(
            (0x0400 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x01f) << 4)
                | (rhs_register & 0x000f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_wo_carry() {
        let lhs_register = 0;
        let lhs_value = 100;
        let rhs_register = 1;
        let rhs_value = 100;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, (lhs_value - rhs_value) as u8);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_wo_carry_zero_before() {
        let lhs_register = 0;
        let lhs_value = 100;
        let rhs_register = 1;
        let rhs_value = 100;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, (lhs_value - rhs_value) as u8);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::Z);

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_with_carry() {
        let lhs_register = 10;
        let lhs_value = 10;
        let rhs_register = 11;
        let rhs_value = 9;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value - rhs_value - 1);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_with_carry_zero_before() {
        let lhs_register = 10;
        let lhs_value = 10;
        let rhs_register = 11;
        let rhs_value = 9;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);
        test_registers.set_status_register_bit(SregBit::Z);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value - rhs_value - 1);
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::Z);

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_zero_before_carry_with_carry() {
        let lhs_register = 15;
        let lhs_value = 127;
        let rhs_register = 16;
        let rhs_value = 127;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(
            lhs_register as usize,
            lhs_value.wrapping_sub(rhs_value).wrapping_sub(1),
        );
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::S);

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative_wo_carry() {
        let lhs_register = 0;
        let lhs_value = 10;
        let rhs_register = 31;
        let rhs_value = 60;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(lhs_register as usize, lhs_value.wrapping_sub(rhs_value));
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::S);

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_negative_with_carry() {
        let lhs_register = 0;
        let lhs_value = 10;
        let rhs_register = 31;
        let rhs_value = 60;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(lhs_register as usize, lhs_value);
        test_registers.set_register(rhs_register as usize, rhs_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(
            lhs_register as usize,
            lhs_value.wrapping_sub(rhs_value).wrapping_sub(1),
        );
        expected_registers.set_register(rhs_register as usize, rhs_value);
        expected_registers.pc = 1;
        expected_registers.set_status_register_bit(SregBit::N);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.set_status_register_bit(SregBit::C);
        expected_registers.set_status_register_bit(SregBit::S);

        let sbc = SBC::new(
            (0x0800 as u16
                | ((rhs_register & 0x0010) << 5)
                | ((lhs_register & 0x001f) << 4)
                | (rhs_register & 0x00f)) as u16,
        );
        sbc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(SBC::get_instruction_codes(), vec![0b0000_1000_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(SBC::get_instruction_mask(), 0xfc00);
    }

    #[test]
    fn test_str() {
        let sbc = SBC::new(0x0bcd);
        assert_eq!(sbc.str(), "sbc r28, r29");
    }
}
