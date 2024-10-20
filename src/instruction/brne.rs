use crate::{instruction::Instruction, registers::Registers};

pub struct BRNE {
    k: i32,
}

impl Instruction for BRNE {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        if registers.sreg_z == false {
            registers.pc += self.k;
        }
    }
    fn str(&self) -> String {
        return format!("brne {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1111_0100_0000_0001]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0111
    }
}

impl BRNE {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: Self::extend(((opcode & 0b0000_0011_1111_1000) >> 3) as i16, 7) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::BRNE;

    #[test]
    fn test_process_true() {
        let k = 7;

        let mut test_registers = Registers::new();
        test_registers.sreg_z = false;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_z = false;
        expected_registers.pc = 1 + k;

        let brne = BRNE::new(0xf001 | (k << 3) as u16);
        brne.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_false() {
        let k = 7;

        let mut test_registers = Registers::new();
        test_registers.sreg_z = true;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_z = true;
        expected_registers.pc = 1;

        let brne = BRNE::new(0xf001 | (k << 3) as u16);
        brne.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(BRNE::get_instruction_codes(), vec![0b1111_0100_0000_0001]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(BRNE::get_instruction_mask(), 0b1111_1100_0000_0111);
    }

    #[test]
    fn test_str() {
        let brne = BRNE::new(0xf7f9);
        assert_eq!(brne.str(), "brne -1");
    }
}
