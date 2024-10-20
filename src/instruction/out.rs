use crate::{instruction::Instruction, registers::Registers};

pub struct OUT {
    r: u16,
    a: u16,
}

impl Instruction for OUT {
    fn process(&self, registers: &mut Registers) {
        registers.pc += 1;
        registers.io[self.a as usize] = registers.r[self.r as usize] as u8
    }
    fn str(&self) -> String {
        return format!("out {}, r{}", self.a, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1011_1000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1000_0000_0000
    }
}

impl OUT {
    pub fn new(opcode: u16) -> Self {
        Self {
            a: ((opcode & 0b0000_0110_0000_0000) >> 5) | (opcode & 0b0000_0000_0000_1111),
            r: (opcode & 0b0000_0001_1111_0000) >> 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, registers::Registers};

    use super::OUT;

    #[test]
    fn test_process() {
        let source_register: u16 = 7;
        let io_location: u16 = 13;
        let source_value = 63;

        let mut test_registers = Registers::new();
        test_registers.r[source_register as usize] = source_value;

        let mut expected_registers = Registers::new();
        expected_registers.io[io_location as usize] = source_value;
        expected_registers.r[source_register as usize] = source_value;
        expected_registers.pc = 1;

        let out = OUT::new(0xb000 | source_register << 4 | io_location);
        out.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(OUT::get_instruction_codes(), vec![0b1011_1000_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(OUT::get_instruction_mask(), 0b1111_1000_0000_0000);
    }

    #[test]
    fn test_str() {
        let out = OUT::new(0xb07a);
        assert_eq!(out.str(), "out 10, r7");
    }
}
