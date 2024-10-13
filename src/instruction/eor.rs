use crate::{instruction::Instruction, registers::Registers};

pub struct EOR {
    d: u16,
    r: u16,
}

impl Instruction for EOR {
    fn process(&self, regisetrs: &mut Registers) {
        regisetrs.r[self.d as usize] ^= regisetrs.r[self.r as usize];
        regisetrs.pc += 1;
    }
    fn str(&self) -> String {
        return format!("eor r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0010_0100_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl EOR {
    const D_MASK: u16 = 0b0000_0001_1111_0000;

    pub fn new(opcode: u16) -> Self {
        let d_value = (opcode & Self::D_MASK) >> 4;
        let r_value = ((opcode & 0b0000_0010_0000_0000) >> 4) | (opcode & 0b0000_0000_0000_1111);

        Self {
            d: d_value,
            r: r_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::EOR;

    #[test]
    fn test_process_same_register() {
        let mut test_registers = Registers::new();
        test_registers.r[3] = 15;

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;

        let eor: EOR = EOR::new(0x2433);
        eor.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_different_registers() {
        let r_register: u16 = 3;
        let d_register: u16 = 4;

        let r_register_value = 7;
        let d_register_value = 9;

        let mut test_registers = Registers::new();
        test_registers.r[r_register as usize] = r_register_value;
        test_registers.r[d_register as usize] = d_register_value;

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.r[r_register as usize] = r_register_value;
        expected_registers.r[d_register as usize] = d_register_value ^ r_register_value;

        let eor: EOR = EOR::new(0x2400 | d_register << 4 | r_register);
        eor.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(EOR::get_instruction_codes(), vec![0x2400]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(EOR::get_instruction_mask(), 0xfc00);
    }

    #[test]
    fn test_str() {
        let rjmp = EOR::new(0x2433);
        assert_eq!(rjmp.str(), "eor r3, r3");

        let rjmp = EOR::new(0x2443);
        assert_eq!(rjmp.str(), "eor r4, r3");
    }
}
