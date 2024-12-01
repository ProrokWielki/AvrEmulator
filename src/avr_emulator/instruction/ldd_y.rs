use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct LDDY {
    q: u16,
    d: u16,
}

impl Instruction for LDDY {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() + 1);
        memory.set_register(
            self.d as usize,
            memory
                .get_sram((memory.get_y_register() + self.q) as usize)
                .unwrap(),
        );
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
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::LDDY;

    #[test]
    fn test_process() {
        let q = 5;
        let d = 8;
        let data = 50;

        let mut test_registers = Memory::new(500, vec![]).unwrap();
        test_registers.set_sram(q as usize, data);

        let mut expected_registers = Memory::new(500, vec![]).unwrap();
        expected_registers.set_register(d as usize, data);
        expected_registers.set_sram(q as usize, data as u8);
        expected_registers.set_pc(1);

        let ldd = LDDY::new(0x8008 | d << 4 | q);
        ldd.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(LDDY::get_instruction_codes(), vec![0b1000_0000_0000_1000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(LDDY::get_instruction_mask(), 0b1101_0010_0000_1000);
    }

    #[test]
    fn test_str() {
        let std = LDDY::new(0x8888);
        assert_eq!(std.str(), "ldd r8, y+16");
    }
}
