use crate::{instruction::Instruction, registers::Registers};

pub struct BRLT {
    k: i32,
}

impl Instruction for BRLT {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        if registers.sreg_n != registers.sreg_v {
            registers.pc += self.k;
        }
    }
    fn str(&self) -> String {
        return format!("brlt {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1111_0000_0000_0100]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0111
    }
}

impl BRLT {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: Self::extend(((opcode & 0b0000_0011_1111_1000) >> 3) as i16, 7) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::BRLT;

    #[test]
    fn test_process_true() {
        let k = 7;

        let mut test_registers = Registers::new();
        test_registers.sreg_n = true;
        test_registers.sreg_v = false;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_n = true;
        expected_registers.sreg_v = false;
        expected_registers.pc = 1 + k;

        let brlt = BRLT::new(0xf004 | (k << 3) as u16);
        brlt.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_false() {
        let k = 7;

        let mut test_registers = Registers::new();
        test_registers.sreg_n = true;
        test_registers.sreg_v = true;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_n = true;
        expected_registers.sreg_v = true;
        expected_registers.pc = 1;

        let brlt = BRLT::new(0xf004 | (k << 3) as u16);
        brlt.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(BRLT::get_instruction_codes(), vec![0b1111_0000_0000_0100]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(BRLT::get_instruction_mask(), 0b1111_1100_0000_0111);
    }

    #[test]
    fn test_str() {
        let brlt = BRLT::new(0xf3fc);
        assert_eq!(brlt.str(), "brlt -1");
    }
}
