use crate::{instruction::Instruction, registers::Registers};

pub struct LDDY {
    q: u16,
    d: u16,
}

impl Instruction for LDDY {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;
        registers.r[self.d as usize] = registers.stack[(registers.y() + self.q) as usize];
    }
    fn str(&self) -> String {
        return format!("ldd r{}, y+{}", self.d, self.q,).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1000_0000_0000_1000]
    }
    fn get_instruction_mask() -> u16 {
        0b1101_0010_0000_1000
    }
}

impl LDDY {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: (opcode & 0b0000_0001_1111_0000) >> 4,
            q: ((opcode & 0b0010_0000_0000_0000) >> 8)
                | ((opcode & 0b0000_1100_0000_0000) >> 7)
                | (opcode & (0b0000_0000_0000_0111)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::LDDY;

    #[test]
    fn test_process() {
        let q = 5;
        let d = 8;
        let data = 50;

        let mut test_registers = Registers::new();
        test_registers.stack[q as usize] = data;

        let mut expected_registers = Registers::new();
        expected_registers.r[d as usize] = data;
        expected_registers.stack[q as usize] = data as u8;
        expected_registers.pc = 1;

        let ldd = LDDY::new(0x8008 | d << 4 | q);
        ldd.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(LDDY::get_instruction_codes(), vec![0b1000_0000_0000_1000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(LDDY::get_instruction_mask(), 0b1101_0010_0000_1000);
    }

    #[test]
    fn test_str() {
        let std = LDDY::new(0x8888);
        assert_eq!(std.str(), "ldd r8, y+16");
    }
}
