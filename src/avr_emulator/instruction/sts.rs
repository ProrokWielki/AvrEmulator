use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct STS {
    r: u16,
    k: u16,
}

impl Instruction for STS {
    fn process(&self, memory: &mut Memory) {
        // TODO: refactor
        let offset = ((memory.get_pc() + 1) * 2) as usize;

        let a = memory.get_flash(offset) as u16;
        let b = memory.get_flash(offset + 1) as u16;

        let k = (b << 8 | a) as u16;

        memory.set_pc(memory.get_pc() + 2);

        memory.set_sram(k as usize, memory.get_register(self.r as usize).unwrap());
        log::info!("k: {}", k);
    }
    fn str(&self) -> String {
        return format!("sts {}, r{}", self.k, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0010_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1110_0000_1111
    }
}

impl STS {
    pub fn new(opcode: u16) -> Self {
        Self {
            r: ((opcode & 0b0000_0001_1111_0000) >> 4),
            k: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::STS;

    #[test]
    fn test_process() {
        let destination_address = 7;
        let source_register = 10;
        let data: u8 = 15;

        let mut test_registers =
            Memory::new(300, vec![0, 0, destination_address, 0, 0, 0]).unwrap();
        test_registers.set_register(source_register as usize, data);

        let mut expected_registers =
            Memory::new(300, vec![0, 0, destination_address, 0, 0, 0]).unwrap();
        expected_registers.set_sram(destination_address as usize, data);
        expected_registers.set_register((source_register) as usize, data as u8);
        expected_registers.set_pc(2);

        let sts = STS::new(0x9000 | source_register << 4);
        sts.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(STS::get_instruction_codes(), vec![0b1001_0010_0000_0000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(STS::get_instruction_mask(), 0b1111_1110_0000_1111);
    }

    #[test]
    fn test_str() {
        let sts = STS::new(0x92f0);
        assert_eq!(sts.str(), "sts 0, r15");
    }
}
