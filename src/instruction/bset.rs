use crate::{instruction::Instruction, registers::Registers};

pub struct BSET {
    s: u8,
}

impl Instruction for BSET {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;

        match self.s {
            0 => registers.sreg_c = true,
            1 => registers.sreg_z = true,
            2 => registers.sreg_n = true,
            3 => registers.sreg_v = true,
            4 => registers.sreg_s = true,
            5 => registers.sreg_h = true,
            6 => registers.sreg_t = true,
            7 => registers.sreg_i = true,
            _ => (),
        }
    }
    fn str(&self) -> String {
        return format!("bset {}", self.s).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0x9408]
    }
    fn get_instruction_mask() -> u16 {
        0xff8f
    }
}

impl BSET {
    pub fn new(opcode: u16) -> Self {
        Self {
            s: ((opcode & 0x0070) >> 4) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::BSET;

    #[test]
    fn test_process() {
        let sreg_bit = 5;

        let mut test_registers = Registers::new();

        let mut expected_registers = Registers::new();
        expected_registers.pc = 1;
        expected_registers.sreg_h = true;

        let bset = BSET::new(0x9408 | (sreg_bit << 4) as u16);
        bset.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(BSET::get_instruction_codes(), vec![0x9408]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(BSET::get_instruction_mask(), 0xff8f);
    }

    #[test]
    fn test_str() {
        let bset = BSET::new(0x9478);
        assert_eq!(bset.str(), "bset 7");
    }
}
