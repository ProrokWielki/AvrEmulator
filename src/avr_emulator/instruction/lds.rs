use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct LDS {
    d: u16,
    k: u16,
}

impl Instruction for LDS {
    fn process(&self, memory: &mut Memory) {
        // TODO: refactor
        let offset = ((memory.get_pc() + 1) * 2) as usize;

        let a = memory.get_flash(offset) as u16;
        let b = memory.get_flash(offset + 1) as u16;

        let k = (b << 8 | a) as u16;

        memory.set_pc(memory.get_pc() + 2);

        memory.set_register(self.d as usize, memory.get_sram(k as usize).unwrap());
        log::info!("k: {}", k);
    }
    fn str(&self) -> String {
        return format!("lds r{}, {}", self.d, self.k).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1110_0000_1111
    }
}

impl LDS {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0b0000_0001_1111_0000) >> 4),
            k: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::LDS;

    #[test]
    fn test_process() {
        let destination_register: u16 = 7;
        let source_address = 10;
        let data: u8 = 15;

        let mut test_registers = Memory::new(300, vec![0, 0, source_address, 0, 0, 0]).unwrap();
        test_registers.set_sram(source_address as usize, data);

        let mut expected_registers = Memory::new(300, vec![0, 0, source_address, 0, 0, 0]).unwrap();
        expected_registers.set_sram(source_address as usize, data);
        expected_registers.set_register((destination_register) as usize, data as u8);
        expected_registers.set_pc(2);

        let lds = LDS::new(0x9000 | destination_register << 4);
        lds.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(LDS::get_instruction_codes(), vec![0b1001_0000_0000_0000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(LDS::get_instruction_mask(), 0b1111_1110_0000_1111);
    }

    #[test]
    fn test_str() {
        let lds = LDS::new(0x90f0);
        assert_eq!(lds.str(), "lds r15, 0");
    }
}
