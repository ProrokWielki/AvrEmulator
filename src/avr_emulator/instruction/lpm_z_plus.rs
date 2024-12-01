use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct LpmZPlus {
    d: u8,
}

impl Instruction for LpmZPlus {
    fn process(&self, memory: &mut Memory) {
        memory.set_pc(memory.get_pc() + 1);

        memory.set_register(
            self.d as usize,
            memory.get_flash(memory.get_z_register() as usize),
        );

        memory.set_z_register(memory.get_z_register() + 1);
    }
    fn str(&self) -> String {
        return format!("lpm r{}, z+", self.d).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b1001_0000_0000_0101]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1110_0000_1111
    }
}

impl LpmZPlus {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x01f0) >> 4) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::LpmZPlus;

    #[test]
    fn test_process() {
        let d = 5;
        let z_pointing_address = 2;
        let flash_value_at_position_z = 127;

        let mut flash = vec![0; 32];
        flash[z_pointing_address] = flash_value_at_position_z;

        let mut test_registers = Memory::new(256, flash.clone()).unwrap();
        test_registers.set_z_register(z_pointing_address as u16);

        let mut expected_registers = Memory::new(256, flash).unwrap();
        expected_registers.set_z_register((z_pointing_address + 1) as u16);
        expected_registers.set_register(d, flash_value_at_position_z);
        expected_registers.set_pc(1);

        let lpm = LpmZPlus::new((0x9005 | d << 4) as u16);
        lpm.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(
            LpmZPlus::get_instruction_codes(),
            vec![0b1001_0000_0000_0101]
        );
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(LpmZPlus::get_instruction_mask(), 0b1111_1110_0000_1111);
    }

    #[test]
    fn test_str() {
        let lpm = LpmZPlus::new(0x91f5);
        assert_eq!(lpm.str(), "lpm r31, z+");
    }
}
