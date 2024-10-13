use crate::{instruction::Instruction, registers::Registers};

pub struct IN {
    d: u16,
    a: u16,
}

impl Instruction for IN {
    fn process(&self, regisetrs: &mut Registers) {
        regisetrs.pc += 1;
        regisetrs.r[self.d as usize] = regisetrs.io[self.a as usize]
    }
    fn str(&self) -> String {
        return format!("in r{}, {}", self.d, self.a).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1011_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1000_0000_0000
    }
}

impl IN {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: (opcode & 0b0000_0001_1111_0000) >> 4,
            a: ((opcode & 0b0000_0110_0000_0000) >> 5) | (opcode & 0b0000_0000_0000_1111),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::IN;

    #[test]
    fn test_process() {
        let destnation_register: u16 = 7;
        let io_location: u16 = 13;
        let io_value = 63;

        let mut test_registers = Registers::new();
        test_registers.io[io_location as usize] = io_value;

        let mut expected_registers = Registers::new();
        expected_registers.io[io_location as usize] = io_value;
        expected_registers.r[destnation_register as usize] = io_value;
        expected_registers.pc = 1;

        let i_in = IN::new(0xb000 | destnation_register << 4 | io_location);
        i_in.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(IN::get_instruction_codes(), vec![0b1011_0000_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(IN::get_instruction_mask(), 0b1111_1000_0000_0000);
    }

    #[test]
    fn test_str() {
        let rjmp = IN::new(0xb07a);
        assert_eq!(rjmp.str(), "in r7, 10");
    }
}
