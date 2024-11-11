use crate::{instruction::Instruction, memory::Memory};

pub struct ADD {
    d: u8,
    r: u8,
}

impl Instruction for ADD {
    fn process(&self, memory: &mut Memory) {
        let result = memory
            .get_register(self.d as usize)
            .unwrap()
            .wrapping_add(memory.get_register(self.r as usize).unwrap());

        memory.update_sreg(
            memory.get_register(self.d as usize).unwrap(),
            memory.get_register(self.r as usize).unwrap(),
            result,
        );

        memory.set_register(self.d as usize, result);

        memory.pc += 1;
    }
    fn str(&self) -> String {
        return format!("add r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0000_1100_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl ADD {
    pub fn new(opcode: u16) -> Self {
        Self {
            d: ((opcode & 0x01f0) >> 4) as u8,
            r: (((opcode & 0x0200) >> 5) | (opcode & 0x000f)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, memory::Memory, memory::SregBit};

    use super::ADD;

    #[test]
    fn test_process() {
        let d_register = 15;
        let r_register = 30;
        let d_value = 50;
        let r_value = 70;

        let mut test_registers = Memory::new(256).unwrap();
        test_registers.set_register(d_register as usize, d_value);
        test_registers.set_register(r_register as usize, r_value);

        let mut expected_registers = Memory::new(256).unwrap();
        expected_registers.set_register(d_register as usize, d_value + r_value);
        expected_registers.set_register(r_register as usize, r_value);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.pc = 1;

        let add = ADD::new(0x0efe);
        add.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(ADD::get_instruction_codes(), vec![0b0000_1100_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(ADD::get_instruction_mask(), 0b1111_1100_0000_0000);
    }

    #[test]
    fn test_str() {
        let add = ADD::new(0x0ef0);
        assert_eq!(add.str(), "add r15, r16");
    }
}
