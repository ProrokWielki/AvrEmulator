use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct ANDI {
    k: u16,
    d: u16,
}

impl Instruction for ANDI {
    fn process(&self, memory: &mut Memory) {
        memory.pc += 1;
        memory.set_register(
            self.d as usize,
            memory.get_register(self.d as usize).unwrap() & self.k as u8,
        );

        memory.clear_status_register_bit(SregBit::V);
        memory.set_status_register_raw_bit_value(
            SregBit::N,
            (memory.get_register(self.d as usize).unwrap() & (1 << 7)) > 0,
        );
        memory.set_status_register_raw_bit_value(
            SregBit::Z,
            memory.get_register(self.d as usize).unwrap() == 0,
        );
        memory.set_status_register_raw_bit_value(
            SregBit::S,
            memory.get_status_register_bit(SregBit::N)
                != memory.get_status_register_bit(SregBit::V),
        );
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
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::ANDI;

    #[test]
    fn test_process_result_0() {
        let k = 8;
        let register = 17;
        let register_value = 16;

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(register, register_value);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_status_register_bit(SregBit::Z);
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

        let mut test_registers = Memory::new(100).unwrap();
        test_registers.set_register(register as usize, register_value as u8);

        let mut expected_registers = Memory::new(100).unwrap();
        expected_registers.set_register(register as usize, (register_value & k) as u8);
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
