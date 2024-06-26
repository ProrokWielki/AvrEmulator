mod add;

pub trait Instruction {
    fn eq(rhs: u16) -> Bool;
    fn perform(self, memory: Memory);
}
