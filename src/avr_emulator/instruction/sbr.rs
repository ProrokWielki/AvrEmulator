use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct SBR {
    d: u16,
    k: u8,
}

impl Instruction for SBR {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() +1);

        memory.set_register(
            self.d as usize,
            memory.get_register(self.d as usize).unwrap() | self.k,
        );
    }
    fn str(&self) -> String {
        return format!("sbr r{}, {}", self.d, self.k).to_owned();
    }

    fn get_instruction_codes() -> Vec<u16> {
        vec![0x6000]
    }
    fn get_instruction_mask() -> u16 {
        0xf000
    }
}

impl SBR {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x00f0) >> 4) + 16,
            k: ((opcode & 0x0f00) >> 4 | (opcode & 0x000f)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::SBR;

    #[test]
    fn test_process_same_register() {
        let k_value = 15;
        let destination_register = 20;
        let register_value = 120;

        let mut test_registers = Memory::new(100, vec![]).unwrap();
        test_registers.set_register(destination_register as usize, register_value);

        let mut expected_registers = Memory::new(100, vec![]).unwrap();
        expected_registers.set_pc(1);
        expected_registers.set_register(destination_register as usize, register_value | k_value);

        let sbr = SBR::new(0x8000 | ((destination_register - 16) << 4) | k_value as u16);
        sbr.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(SBR::get_instruction_codes(), vec![0x6000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(SBR::get_instruction_mask(), 0xf000);
    }

    #[test]
    fn test_str() {
        let sbr = SBR::new(0x6fff);
        assert_eq!(sbr.str(), "sbr r31, 255");
    }
}
