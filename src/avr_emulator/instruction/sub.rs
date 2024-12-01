use crate::avr_emulator::{instruction::Instruction, memory::Memory};

pub struct SUB {
    d: u8,
    r: u8,
}

impl Instruction for SUB {
    fn process(&self, memory: &mut Memory) {
        let result = memory
            .get_register(self.d as usize)
            .unwrap()
            .wrapping_sub(memory.get_register(self.r as usize).unwrap());

        memory.update_sreg(
            memory.get_register(self.d as usize).unwrap(),
            memory.get_register(self.r as usize).unwrap(),
            result,
        );

        memory.set_register(self.d as usize, result);

        memory.set_pc(memory.get_pc() + 1);
    }
    fn str(&self) -> String {
        return format!("sub r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0001_1000_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl SUB {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x01f0) >> 4) as u8,
            r: (((opcode & 0x0200) >> 5) | (opcode & 0x000f)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avr_emulator::{instruction::Instruction, memory::Memory};

    use super::SUB;

    #[test]
    fn test_process() {
        let d_register = 15;
        let r_register = 30;
        let d_value = 70;
        let r_value = 50;

        let mut test_registers = Memory::new(256, vec![]).unwrap();
        test_registers.set_register(d_register as usize, d_value);
        test_registers.set_register(r_register as usize, r_value);

        let mut expected_registers = Memory::new(256, vec![]).unwrap();
        expected_registers.set_register(d_register as usize, d_value - r_value);
        expected_registers.set_register(r_register as usize, r_value);
        expected_registers.set_pc(1);

        let sub = SUB::new(0x0efe);
        sub.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_get_instruction_codes() {
        assert_eq!(SUB::get_instruction_codes(), vec![0b0001_1000_0000_0000]);
    }

    #[test]
    fn test_get_instruction_mask() {
        assert_eq!(SUB::get_instruction_mask(), 0b1111_1100_0000_0000);
    }

    #[test]
    fn test_str() {
        let sub = SUB::new(0x1af0);
        assert_eq!(sub.str(), "sub r15, r16");
    }
}
