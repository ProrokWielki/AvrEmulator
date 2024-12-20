use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct RETI {}

impl Instruction for RETI {
    fn process(&self, memory: &mut Memory) {
        memory.set_sp(memory.get_sp() + 2);
        memory.set_pc(
            ((memory.get_stack((memory.get_sp() - 1) as usize).unwrap() as u16) << 8)
                | (memory.get_stack(memory.get_sp() as usize).unwrap() as u16),
        );

        memory.set_status_register_bit(SregBit::I);
    }
    fn str(&self) -> String {
        return format!("reti").to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0101_0001_1000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1111_1111_1111
    }
}

impl RETI {
    pub fn new(_opcode: u16) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::RETI;

    #[test]
    fn test_process() {
        let start_sp = 10;
        let expected_sp = start_sp + 2;
        let expected_pc: u16 = 13;

        let mut test_registers = Memory::new(200, vec![]).unwrap();
        test_registers.set_sp(start_sp);
        test_registers.set_stack(expected_sp as usize, expected_pc as u8);

        let mut expected_registers = Memory::new(200, vec![]).unwrap();
        expected_registers.set_pc(expected_pc);
        expected_registers.set_sp(expected_sp);
        expected_registers.set_stack(expected_sp as usize, expected_pc as u8);
        expected_registers.set_status_register_bit(SregBit::I);

        let reti = RETI::new(0b1001_0101_0001_1000);
        reti.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(RETI::get_instruction_codes(), vec![0b1001_0101_0001_1000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(RETI::get_instruction_mask(), 0xffff);
    }

    #[test]
    fn test_str() {
        let reti = RETI::new(0b1001_0101_0001_1000);

        assert_eq!(reti.str(), "reti");
    }
}
