use crate::{instruction::Instruction, memory::Memory, memory::SregBit};

pub struct ADC {
    d: u8,
    r: u8,
}

impl Instruction for ADC {
    fn process(&self, memory: &mut Memory) {
        let result = memory
            .get_register(self.d as usize)
            .unwrap()
            .wrapping_add(memory.get_register(self.r as usize).unwrap())
            .wrapping_add(if memory.get_status_register_bit(SregBit::C) {
                1
            } else {
                0
            });

        memory.update_sreg(
            memory.get_register(self.d as usize).unwrap(),
            memory.get_register(self.r as usize).unwrap(),
            result,
        );

        memory.set_register(self.d as usize, result);

        memory.pc += 1;
    }
    fn str(&self) -> String {
        return format!("adc r{}, r{}", self.d, self.r).to_owned();
    }
    fn get_instruction_codes() -> Vec<u16> {
        vec![0b0001_1100_0000_0000]
    }
    fn get_instruction_mask() -> u16 {
        0b1111_1100_0000_0000
    }
}

impl ADC {
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

    use super::ADC;

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

        let add = ADC::new(0x0efe);
        add.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn test_process_with_carry() {
        let d_register = 15;
        let r_register = 30;
        let d_value = 50;
        let r_value = 70;

        let mut test_registers = Memory::new(256).unwrap();
        test_registers.set_register(d_register as usize, d_value);
        test_registers.set_register(r_register as usize, r_value);
        test_registers.set_status_register_bit(SregBit::C);

        let mut expected_registers = Memory::new(256).unwrap();
        expected_registers.set_register(d_register as usize, d_value + r_value + 1);
        expected_registers.set_register(r_register as usize, r_value);
        expected_registers.set_status_register_bit(SregBit::H);
        expected_registers.pc = 1;

        let adc = ADC::new(0x1efe);
        adc.process(&mut test_registers);

        assert_eq!(test_registers, expected_registers);
    }

    #[test]
    fn tests_get_instraction_codes() {
        assert_eq!(ADC::get_instruction_codes(), vec![0b0001_1100_0000_0000]);
    }

    #[test]
    fn tests_get_instraction_mask() {
        assert_eq!(ADC::get_instruction_mask(), 0b1111_1100_0000_0000);
    }

    #[test]
    fn test_str() {
        let add = ADC::new(0x1ef0);
        assert_eq!(add.str(), "adc r15, r16");
    }
}
