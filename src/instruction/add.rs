use crate::memory::memory::IMemory;
use crate::memory::memory::Memory;

pub struct ADD {
    destination_register: u16,
    source_register: u16,
}

impl ADD {
    const opcode: u16 = 0b0000110000000000;
    const opcode_mask: u16 = 0b1111110000000000;
    pub fn new(instruction: u16) -> Self {
        Self {
            source_register: ((instruction & (1 << 9)) >> 5) | instruction & (0b1111),
            destination_register: (instruction & (0b111110000)) >> 4,
        }
    }
    pub fn eq(rhs: u16) -> bool {
        return (rhs & ADD::opcode_mask) == ADD::opcode;
    }
    pub fn perform(&self, memory: &mut impl IMemory) {
        *memory.get(self.destination_register) += *memory.get(self.source_register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    mock! {
        pub Memory{}
        impl IMemory for Memory
        {
            fn get(&mut self, address: u16) -> &mut u16;
        }
    }

    #[test]
    fn test_new() {
        let add = ADD::new(0b0000111010100101);
        assert_eq!(add.destination_register, 0b01010);
        assert_eq!(add.source_register, 0b10101);
    }

    #[test]
    fn test_eq() {
        assert_eq!(ADD::eq(0b0000110000000000), true);
        assert_eq!(ADD::eq(0b0000010000000000), false);
        assert_eq!(ADD::eq(0b0000100000000000), false);
        assert_eq!(ADD::eq(0b0001110000000000), false);

        assert_eq!(ADD::eq(0b0000111111111111), true);
    }

    #[ignore]
    #[test]
    fn test_perform_with_mock() {
        let mut add = ADD::new(0b0000111010100101);
        let mut mock_memory = MockMemory::new();

        let mut source_address: u16 = 0b10101;
        let mut destination_address: u16 = 0b01010;

        let &mut source_value: &mut u16 = &mut 5;
        let &mut destination_value: &mut u16 = &mut 7;

        mock_memory
            .expect_get()
            .times(1)
            .with(predicate::eq(source_address))
            .return_var(source_value);

        mock_memory
            .expect_get()
            .times(1)
            .with(predicate::eq(destination_address))
            .return_var(destination_value);

        add.perform(&mut mock_memory);

        assert_eq!(destination_value, source_value + destination_value);
    }

    #[test]
    fn test_perform() {
        let mut add = ADD::new(0b0000111010100101);
        let mut mock_memory = Memory::new(0b01010, 20);

        let mut source_address: u16 = 0b10101;
        let mut destination_address: u16 = 0b01010;

        let mut source_value: u16 = 5;
        let mut destination_value: u16 = 7;

        *mock_memory.get(source_address) = source_value;
        *mock_memory.get(destination_address) = destination_value;

        add.perform(&mut mock_memory);

        assert_eq!(
            *mock_memory.get(destination_address),
            destination_value + source_value
        );
    }
}
