use crate::{instruction::Instruction, registers::Registers};

pub struct ANDI {
    k: u16,
    d: u16,
}

impl Instruction for ANDI {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;
        registers.r[self.d as usize] = registers.r[self.d as usize] & self.k as u8;

        registers.sreg_v = false;
        registers.sreg_n = (registers.r[self.d as usize] & (1 << 7)) > 0;
        registers.sreg_z = registers.r[self.d as usize] == 0;
        registers.sreg_s = registers.sreg_n != registers.sreg_v;
    }
    fn str(&self) -> String {
        return format!("andi r{}, {}", self.d, self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0111_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_0000_0000_0000
    }
}

impl ANDI {
    pub fn new(opcode: u16) -> Self {
        Self {
            k: ((opcode & 0x0f00) >> 4) | (opcode & 0x000f),
            d: ((opcode & 0x00f0) >> 4) + 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::ANDI;

    #[test]
    fn test_process_result_0() {
        let k = 8;
        let register = 17;
        let register_value = 16;

        let mut test_registers = Registers::new();
        test_registers.r[register] = register_value;

        let mut expected_registers = Registers::new();
        expected_registers.sreg_z = true;
        expected_registers.pc = 1;

        let andi = ANDI::new((0x7000 | ((register - 16) << 4) | k) as u16);
        andi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_result_not_0() {
        let k: u16 = 17;
        let register = 17;
        let register_value: u16 = 1;

        let mut test_registers = Registers::new();
        test_registers.r[register as usize] = register_value as u8;

        let mut expected_registers = Registers::new();
        expected_registers.r[register as usize] = (register_value & k) as u8;
        expected_registers.pc = 1;

        let andi = ANDI::new((0x7000 | ((register - 16) << 4) | k) as u16);
        andi.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instruction_codes() {
        assert_eq!(ANDI::get_instruction_codes(), vec![0b0111_0000_0000_0000]);
    }

    #[test]
    fn tests_get_instruction_mask() {
        assert_eq!(ANDI::get_instruction_mask(), 0b1111_0000_0000_0000);
    }

    #[test]
    fn test_str() {
        let andi = ANDI::new(0x7841);
        assert_eq!(andi.str(), "andi r20, 129");
    }
}
