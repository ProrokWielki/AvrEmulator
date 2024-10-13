use crate::registers::Registers;

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
}
