use crate::{instruction::Instruction, registers::Registers};

pub struct BREQ {
    k: i32,
}

impl Instruction for BREQ {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        if registers.sreg_z {
            registers.pc += self.k;
        }
    }
    fn str(&self) -> String {
        return format!("breq {}", self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1111_0000_0000_0001]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0111
    }
}

impl BREQ {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: Self::extend(((opcode & 0b0000_0011_1111_1000) >> 3) as i16, 7) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::BREQ;

    #[test]
    fn test_process_true() {
        let k = 7;

        let mut test_registers = Registers::new();
        test_registers.sreg_z = true;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_z = true;
        expected_registers.pc = 1 + k;

        let breq = BREQ::new(0xf001 | (k << 3) as u16);
        breq.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_false() {
        let k = 7;

        let mut test_registers = Registers::new();
        test_registers.sreg_z = false;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_z = false;
        expected_registers.pc = 1;

        let breq = BREQ::new(0xf001 | (k << 3) as u16);
        breq.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(BREQ::get_instruction_codes(), vec![0b1111_0000_0000_0001]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(BREQ::get_instruction_mask(), 0b1111_1100_0000_0111);
    }

    #[test]
    fn test_str() {
        let breq = BREQ::new(0xf3f9);
        assert_eq!(breq.str(), "breq -1");
    }
}
