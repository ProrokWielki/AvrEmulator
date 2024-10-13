use crate::registers::Registers;

mod nop;
mod ret;
mod rjmp;
pub trait Instruction {
    fn process(&self, registers: &mut Registers) -> ();
    fn str(&self) -> String;

    fn get_instruction_codes() -> Vec<u16>
    where
        Self: Sized;
    fn get_instruction_mask() -> u16
    where
        Self: Sized;

    fn eq(opcode: u16) -> bool
    where
        Self: Sized,
    {
        for instcruction_code in Self::get_instruction_codes() {
            if opcode & Self::get_instruction_mask() == instcruction_code {
                return true;
            }
        }
        false
    }

    fn extend(value: i16, orginal_length: u8) -> i16
    where
        Self: Sized,
    {
        let mask = 1 << (orginal_length - 1);
        (value ^ mask) - mask
    }
}

pub fn get_instruction(opcode: u16) -> Option<Box<dyn Instruction>> {
    if nop::NOP::eq(opcode) {
        return Some(Box::new(nop::NOP::new(opcode)));
    }
    if ret::RET::eq(opcode) {
        return Some(Box::new(ret::RET::new(opcode)));
    }
    if rjmp::RJMP::eq(opcode) {
        return Some(Box::new(rjmp::RJMP::new(opcode)));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockInstruction {}

    impl Instruction for MockInstruction {
        fn process(&self, _regisetrs: &mut Registers) {}
        fn str(&self) -> String {
            return "mock".to_owned();
        }
        fn get_instruction_codes() -> Vec<u16> {
            vec![0xf0f0]
        }
        fn get_instruction_mask() -> u16 {
            0xfff0
        }
    }

    #[test]
    fn test_eq_returns_true_for_valid_instraction() {
        assert!(MockInstruction::eq(0xf0f0));
        assert!(MockInstruction::eq(0xf0ff));
    }

    #[test]
    fn test_eq_returns_false_for_invalid_instraction() {
        assert!(!MockInstruction::eq(0x0000));
        assert!(!MockInstruction::eq(0xf000));
        assert!(!MockInstruction::eq(0x00f0));
        assert!(!MockInstruction::eq(0xe0e0));
    }

    #[test]
    fn test_extend_return_correct_value_for_positiv_integers() {
        assert_eq!(MockInstruction::extend(0x0001, 8), 1);
        assert_eq!(MockInstruction::extend(0x00ff, 9), 255);
        assert_eq!(MockInstruction::extend(0x0003, 4), 3);
    }

    #[test]
    fn test_extend_return_correct_value_for_negative_integers() {
        assert_eq!(MockInstruction::extend(0x0003, 2), -1);
        assert_eq!(MockInstruction::extend(0x00ff, 8), -1);
        assert_eq!(MockInstruction::extend(0x0100, 9), -256);
    }

    #[test]
    fn test_get_instruction_retunrs_none_for_invalid_opcode() {
        assert!(get_instruction(0xffff).is_none());
    }

    #[test]
    fn test_get_instruction_retunrs_nop_for_nop_opcode() {
        assert_eq!(get_instruction(0x0000).unwrap().str(), "nop");
    }

    #[test]
    fn test_get_instruction_retunrs_ret_for_ret_opcode() {
        assert_eq!(get_instruction(0b1001_0101_0000_1000).unwrap().str(), "ret");
    }

    #[test]
    fn test_get_instruction_retunrs_rjmp_for_rjmp_opcode() {
        assert_eq!(get_instruction(0xcfff).unwrap().str(), "rjmp -1");
    }
}
