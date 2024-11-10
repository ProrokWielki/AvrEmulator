use crate::{instruction::Instruction, memory::Memory};

pub struct STDY {
    q: u16,
    r: u16,
}

impl Instruction for STDY {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;

        memory.set_stack(
            (memory.get_y_register() + self.q) as usize,
            memory.get_register(self.r as usize).unwrap(),
        );
    }
    fn str(&self) -> String {
        return format!("std y+{}, r{}", self.q, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1000_0010_0000_1000]
    }
    fn get_instruction_mask() -> u16 {
        0b1101_0010_0000_1000
    }
}

impl STDY {
    pub fn new(opcode: u16) -> Self {
        Self {
            r: (opcode & 0b0000_0001_1111_0000) >> 4,
            q: ((opcode & 0b0010_0000_0000_0000) >> 8)
                | ((opcode & 0b0000_1100_0000_0000) >> 7)
                | (opcode & (0b0000_0000_0000_0111)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, memory::Memory};

    use super::STDY;

    #[test]
    fn test_process() {
        let q = 5;
        let r = 8;
        let data = 50;

        let mut test_registers = Memory::new(256).unwrap();
        test_registers.set_register(r as usize, data);

        let mut expected_registers = Memory::new(256).unwrap();
        expected_registers.set_register(r as usize, data);
        expected_registers.set_stack(q as usize, data as u8);
        expected_registers.pc = 1;

        let std = STDY::new(0x8208 | r << 4 | q);
        std.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(STDY::get_instruction_codes(), vec![0b1000_0010_0000_1000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(STDY::get_instruction_mask(), 0b1101_0010_0000_1000);
    }

    #[test]
    fn test_str() {
        let std = STDY::new(0x8a88);
        assert_eq!(std.str(), "std y+16, r8");
    }
}
