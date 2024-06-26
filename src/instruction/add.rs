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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
