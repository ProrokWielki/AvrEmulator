use crate::{instruction::Instruction, registers::Registers};

pub struct BRBS {
    s: u8,
    k: i32,
}

impl Instruction for BRBS {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        let mut bit_set = false;

        match self.s {
            0 => bit_set = registers.sreg_c,
            1 => bit_set = registers.sreg_z,
            2 => bit_set = registers.sreg_n,
            3 => bit_set = registers.sreg_v,
            4 => bit_set = registers.sreg_s,
            5 => bit_set = registers.sreg_h,
            6 => bit_set = registers.sreg_t,
            7 => bit_set = registers.sreg_i,
            _ => (),
        }

        if bit_set {
            registers.pc += self.k;
        }
    }
    fn str(&self) -> String {
        return format!("brbs {}, {}", self.s, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0xf000]
    }
    fn get_instruction_mask() -> u16 {
        0xfc00
    }
}

impl BRBS {
    pub fn new(opcode: u16) -> Self {
        Self {
            s: (opcode & 0x0007) as u8,
            k: Self::extend(((opcode & 0x03f8) >> 3) as i16, 7) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::BRBS;

    #[test]
    fn test_process() {
        let sreg_bit = 5;
        let k = 15;

        let mut test_registers = Registers::new();
        test_registers.sreg_h = true;

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1 + k;
        expected_registers.sreg_h = true;

        let brbs = BRBS::new((0xf000 | (k << 3) | (sreg_bit)) as u16);
        brbs.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(BRBS::get_instruction_codes(), vec![0xf000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(BRBS::get_instruction_mask(), 0xfc00);
    }

    #[test]
    fn test_str() {
        let brbs = BRBS::new(0xf3fd);
        assert_eq!(brbs.str(), "brbs 5, -1");
    }
}
