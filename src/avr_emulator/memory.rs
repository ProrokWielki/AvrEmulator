#[derive(Debug, PartialEq, Clone)]
pub struct Memory {
    sram: Vec<u8>,
    pc: u16,
    flash: Vec<u8>,
}

pub enum SregBit {
    I,
    T,
    H,
    S,
    V,
    N,
    Z,
    C,
}

impl Memory {
    pub const REGISTERS_START: usize = 0;
    pub const REGISTERS_SIZE: usize = 32;

    pub const IO_START: usize = Self::REGISTERS_START + Self::REGISTERS_SIZE;
    pub const IO_SIZE: usize = 64;

    pub const STACK_START: usize = Self::IO_START + Self::IO_SIZE;

    pub fn new(size: usize, flash: Vec<u8>) -> Result<Self, String> {
        if size < Self::STACK_START {
            return Err("Size to small".to_owned());
        }
        Ok(Self {
            sram: vec![0; size],
            pc: 0,
            flash: flash,
        })
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, new_pc: u16) {
        self.pc = new_pc;
    }

    pub fn get_all_registers(&self) -> Vec<u8> {
        self.sram[Self::REGISTERS_START..Self::REGISTERS_START + Self::REGISTERS_SIZE].to_vec()
    }
    pub fn get_all_io(&self) -> Vec<u8> {
        self.sram[Self::IO_START..Self::IO_START + Self::IO_SIZE].to_vec()
    }
    pub fn get_all_stack(&self) -> Vec<u8> {
        self.sram[Self::STACK_START..].to_vec()
    }

    pub fn set_register(&mut self, register: usize, value: u8) {
        if register >= Self::REGISTERS_SIZE {
            panic!("Trying to access register out of bounds");
        }
        self.set_sram(register, value);
    }

    pub fn get_register(&self, register: usize) -> Result<u8, String> {
        if register >= Self::REGISTERS_SIZE {
            return Err("Trying to access register out of bounds".to_owned());
        }
        self.get_sram(register)
    }

    pub fn set_io(&mut self, io: usize, value: u8) {
        if io >= Self::IO_SIZE {
            panic!("Trying to access io register out of bounds");
        }
        self.set_sram(io + Self::IO_START, value);
    }

    pub fn get_io(&self, io: usize) -> Result<u8, String> {
        if io >= Self::IO_SIZE {
            return Err("Trying to access io register out of bounds".to_owned());
        }
        self.get_sram(io + Self::IO_START)
    }

    pub fn set_stack(&mut self, address: usize, value: u8) {
        if address >= self.sram.len() - Self::STACK_START {
            panic!("Trying to access stack memory out of bounds");
        }
        self.sram[address + Self::STACK_START] = value;
    }

    pub fn get_stack(&self, address: usize) -> Result<u8, String> {
        if address >= self.sram.len() - Self::STACK_START {
            return Err("Trying to access stack memory out of bounds".to_owned());
        }
        Ok(self.sram[address + Self::STACK_START])
    }

    pub fn get_flash(&self, address: usize) -> u8 {
        if address >= self.flash.len() {
            panic!("Trying to access stack memory out of bounds");
        }
        self.flash[address]
    }

    pub fn set_sram(&mut self, address: usize, value: u8) {
        if address >= self.sram.len() {
            panic!("Trying to access sram memory out of bounds: {}", address);
        }
        self.sram[address] = value;
    }

    pub fn get_sram(&self, address: usize) -> Result<u8, String> {
        if address >= self.sram.len() {
            return Err("Trying to access sram memory out of bounds".to_owned());
        }
        Ok(self.sram[address])
    }

    pub fn get_as_16bit(&self, address: usize) -> Result<u16, String> {
        let msb = self.get_sram(address + 1);
        let lsb = self.get_sram(address);

        if msb.is_err() || lsb.is_err() {
            return Err("Trying to access address out of bounds".to_owned());
        }

        Ok(((msb.unwrap() as u16) << 8) | lsb.unwrap() as u16)
    }
    pub fn set_as_16bit(&mut self, address: usize, new_value: u16) {
        self.set_sram(address + 1, (new_value >> 8) as u8);
        self.set_sram(address, (new_value & 0x00ff) as u8);
    }

    pub fn get_sp(&self) -> u16 {
        self.get_as_16bit(93).unwrap()
    }
    pub fn set_sp(&mut self, new_sp: u16) {
        self.set_as_16bit(93, new_sp);
    }

    pub fn get_x_register(&self) -> u16 {
        self.get_as_16bit(26).unwrap()
    }
    pub fn set_x_register(&mut self, new_x: u16) {
        self.set_as_16bit(26, new_x);
    }

    pub fn get_y_register(&self) -> u16 {
        self.get_as_16bit(28).unwrap()
    }
    pub fn set_y_register(&mut self, new_y: u16) {
        self.set_as_16bit(28, new_y);
    }

    pub fn get_z_register(&self) -> u16 {
        self.get_as_16bit(30).unwrap()
    }
    pub fn set_z_register(&mut self, new_z: u16) {
        self.set_as_16bit(30, new_z);
    }

    pub fn get_status_register(&self) -> u8 {
        self.get_io(63).unwrap()
    }
    pub fn set_status_register(&mut self, value: u8) {
        self.set_io(63, value);
    }
    pub fn get_status_register_bit(&self, bit: SregBit) -> bool {
        Self::bit8(self.get_status_register(), Self::to_bit_position(bit))
    }
    pub fn set_status_register_raw_bit_value(&mut self, bit: SregBit, value: bool) {
        if value {
            self.set_status_register_bit(bit);
        } else {
            self.clear_status_register_bit(bit);
        }
    }
    pub fn set_status_register_bit(&mut self, bit: SregBit) {
        self.set_status_register(self.get_status_register() | (1 << Self::to_bit_position(bit)));
    }
    pub fn clear_status_register_bit(&mut self, bit: SregBit) {
        self.set_status_register(self.get_status_register() & (!(1 << Self::to_bit_position(bit))));
    }

    pub fn update_sreg(&mut self, lhs: u8, rhs: u8, result: u8) {
        self.set_status_register_raw_bit_value(
            SregBit::H,
            (!Self::bit8(lhs, 3) && Self::bit8(rhs, 3))
                || (Self::bit8(rhs, 3) && Self::bit8(result, 3))
                || (Self::bit8(result, 3) && !Self::bit8(lhs, 3)),
        );

        self.set_status_register_raw_bit_value(
            SregBit::V,
            (Self::bit8(lhs, 7) && !Self::bit8(rhs, 7) && !Self::bit8(rhs, 7))
                || (!Self::bit8(lhs, 7) && Self::bit8(rhs, 7) && Self::bit8(result, 7)),
        );

        self.set_status_register_raw_bit_value(SregBit::N, Self::bit8(result, 7));

        self.set_status_register_raw_bit_value(SregBit::Z, result == 0);

        self.set_status_register_raw_bit_value(
            SregBit::C,
            (!Self::bit8(lhs, 7) && Self::bit8(rhs, 7))
                || (Self::bit8(rhs, 7) && Self::bit8(result, 7))
                || (Self::bit8(result, 7) && !Self::bit8(lhs, 7)),
        );

        self.set_status_register_raw_bit_value(
            SregBit::S,
            self.get_status_register_bit(SregBit::N) != self.get_status_register_bit(SregBit::V),
        );
    }

    pub fn update_sreg_keep_z_if_result_zero(&mut self, lhs: u8, rhs: u8, result: u8) {
        let old_z = self.get_status_register_bit(SregBit::Z);

        self.update_sreg(lhs, rhs, result);

        if result == 0 {
            self.set_status_register_raw_bit_value(SregBit::Z, old_z);
        } else {
            self.clear_status_register_bit(SregBit::Z);
        }
    }

    pub fn update_sreg_16bit(&mut self, lhs: u16, _rhs: u16, result: u16) {
        self.set_status_register_raw_bit_value(
            SregBit::V,
            Self::bit16(result, 15) && !Self::bit16(lhs, 15),
        );

        self.set_status_register_raw_bit_value(SregBit::N, Self::bit16(result, 15));

        self.set_status_register_raw_bit_value(SregBit::Z, result == 0);

        self.set_status_register_raw_bit_value(
            SregBit::C,
            Self::bit16(result, 15) && !Self::bit16(lhs, 15),
        );

        self.set_status_register_raw_bit_value(
            SregBit::S,
            self.get_status_register_bit(SregBit::N) != self.get_status_register_bit(SregBit::V),
        );
    }

    fn bit16(var: u16, bit: u16) -> bool {
        (var & (1 << bit)) != 0
    }

    fn bit8(var: u8, bit: u8) -> bool {
        Self::bit16(var as u16, bit as u16)
    }

    fn to_bit_position(sreg_bit: SregBit) -> u8 {
        match sreg_bit {
            SregBit::I => 7,
            SregBit::T => 6,
            SregBit::H => 5,
            SregBit::S => 4,
            SregBit::V => 3,
            SregBit::N => 2,
            SregBit::Z => 1,
            SregBit::C => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit8() {
        let test_value = 0b1011_0010;

        for i in 0..7 {
            assert_eq!(Memory::bit8(test_value, i), (test_value & (1 << i)) != 0);
        }
    }

    #[test]
    fn test_bit16() {
        let test_value = 0xabcd;

        for i in 0..15 {
            assert_eq!(Memory::bit16(test_value, i), (test_value & (1 << i)) != 0);
        }
    }

    #[test]
    fn test_get_as_16bit() {
        let msb: u8 = 0xf0;
        let lsb: u8 = 0x5a;
        let register = 12;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.sram[register + 1] = msb;
        memory.sram[register] = lsb;

        assert_eq!(
            memory.get_as_16bit(register).unwrap(),
            ((msb as u16) << 8) | lsb as u16
        );
    }

    #[test]
    fn test_get_as_16bit_out_of_bounds() {
        let memory = Memory::new(100, vec![]).unwrap();

        assert!(memory.get_as_16bit(101).is_err());
    }

    #[test]
    fn test_set_as_16bit() {
        let data: u16 = 0xf05a;
        let register = 12;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_as_16bit(register, data);

        assert_eq!(memory.sram[register], (data & 0x00ff) as u8);
        assert_eq!(memory.sram[register + 1], (data >> 8) as u8);
    }

    #[test]
    #[should_panic]
    fn test_set_register_out_of_bounds() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.set_register(32, 0);
    }

    #[test]
    fn test_new_fails_on_too_small_size() {
        let memory = Memory::new(10, vec![]);

        assert!(memory.is_err());
    }

    #[test]
    fn test_get_register_fails_on_out_of_bounds_access() {
        let memory = Memory::new(100, vec![]).unwrap();

        assert!(memory.get_register(64).is_err());
    }

    #[test]
    fn test_get_register_returns_correct_value() {
        let register = 12;
        let register_value = 17;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.sram[register] = register_value;

        assert_eq!(memory.get_register(register).unwrap(), register_value);
    }

    #[test]
    fn test_get_io_register_fails_on_out_of_bounds_access() {
        let memory = Memory::new(100, vec![]).unwrap();

        assert!(memory.get_io(100).is_err());
    }

    #[test]
    fn test_get_io_register_returns_correct_value() {
        let register = 12;
        let register_value = 17;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.sram[32 + register] = register_value;

        assert_eq!(memory.get_io(register).unwrap(), register_value);
    }

    #[test]
    #[should_panic]
    fn test_set_io_register_out_of_bounds() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.set_io(65, 0);
    }

    #[test]
    fn test_get_sram_fails_on_out_of_bounds_access() {
        let memory = Memory::new(100, vec![]).unwrap();

        assert!(memory.get_sram(100).is_err());
    }

    #[test]
    fn test_get_sram_returns_correct_value() {
        let address = 99;
        let address_value = 17;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.sram[address] = address_value;

        assert_eq!(memory.get_sram(address).unwrap(), address_value);
    }

    #[test]
    #[should_panic]
    fn test_set_sram_out_of_bounds() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.set_sram(101, 0);
    }

    #[test]
    fn test_set_sram() {
        let address = 99;
        let value = 50;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_sram(address, value);

        assert_eq!(memory.sram[address], value);
    }

    #[test]
    #[should_panic]
    fn test_set_stack_out_of_bounds() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.set_stack(101, 0);
    }

    #[test]
    fn test_get_stack_out_of_bounds() {
        let memory = Memory::new(100, vec![]).unwrap();

        assert!(memory.get_stack(101).is_err());
    }

    #[test]
    fn test_get_sp() {
        let lsb: u8 = 0xa5;
        let msb: u8 = 0xfa;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_io(61, lsb);
        memory.set_io(62, msb);

        assert_eq!(memory.get_sp(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_sp() {
        let new_sp = 0x8f12;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_sp(new_sp);

        assert_eq!(memory.get_sp(), new_sp);
    }

    #[test]
    fn test_get_x_register() {
        let lsb: u8 = 0x18;
        let msb: u8 = 0x81;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_register(26, lsb);
        memory.set_register(27, msb);

        assert_eq!(memory.get_x_register(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_x_register() {
        let new_x = 0xf0f0;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_x_register(new_x);

        assert_eq!(memory.get_x_register(), new_x);
    }

    #[test]
    fn test_get_y_register() {
        let lsb: u8 = 0x12;
        let msb: u8 = 0x34;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.sram[28] = lsb;
        memory.sram[29] = msb;

        assert_eq!(memory.get_y_register(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_y_register() {
        let new_y = 0xdead;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_y_register(new_y);

        assert_eq!(memory.get_y_register(), new_y);
    }

    #[test]
    fn test_get_z_register() {
        let lsb: u8 = 0x43;
        let msb: u8 = 0x21;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.sram[30] = lsb;
        memory.sram[31] = msb;

        assert_eq!(memory.get_z_register(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_z_register() {
        let new_z = 0xbeef;

        let mut memory = Memory::new(100, vec![]).unwrap();
        memory.set_z_register(new_z);

        assert_eq!(memory.get_z_register(), new_z);
    }

    #[test]
    fn test_sreg_update_borrow_from_bit_3() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg(8, 2, 3);
        assert_eq!(memory.get_status_register_bit(SregBit::H), false);

        memory.update_sreg(2, 9, 2);
        assert_eq!(memory.get_status_register_bit(SregBit::H), true);

        memory.update_sreg(10, 11, 12);
        assert_eq!(memory.get_status_register_bit(SregBit::H), true);

        memory.update_sreg(3, 2, 8);
        assert_eq!(memory.get_status_register_bit(SregBit::H), true);
    }

    #[test]
    fn test_sreg_update_result_zero_no_keep() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg(1, 2, 3);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), false);

        memory.update_sreg(2, 1, 0);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), true);
    }

    #[test]
    fn test_sreg_update_result_zero_keep() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.set_status_register_bit(SregBit::Z);
        memory.update_sreg_keep_z_if_result_zero(1, 2, 3);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), false);

        memory.set_status_register_bit(SregBit::Z);
        memory.update_sreg_keep_z_if_result_zero(1, 2, 0);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), true);

        memory.clear_status_register_bit(SregBit::Z);
        memory.update_sreg_keep_z_if_result_zero(1, 2, 3);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), false);

        memory.clear_status_register_bit(SregBit::Z);
        memory.update_sreg_keep_z_if_result_zero(2, 1, 0);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), false);
    }

    #[test]
    fn test_sreg_update_carry_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg(128, 2, 3);
        assert_eq!(memory.get_status_register_bit(SregBit::C), false);

        memory.update_sreg(2, 128, 0);
        assert_eq!(memory.get_status_register_bit(SregBit::C), true);

        memory.update_sreg(128, 129, 130);
        assert_eq!(memory.get_status_register_bit(SregBit::C), true);

        memory.update_sreg(2, 1, 131);
        assert_eq!(memory.get_status_register_bit(SregBit::C), true);
    }

    #[test]
    fn test_sreg_update_n_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg(0, 0, 127);
        assert_eq!(memory.get_status_register_bit(SregBit::N), false);

        memory.update_sreg(0, 0, 128);
        assert_eq!(memory.get_status_register_bit(SregBit::N), true);
    }

    #[test]
    fn test_sreg_update_v_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg(0, 1, 2);
        assert_eq!(memory.get_status_register_bit(SregBit::V), false);

        memory.update_sreg(127, 128, 129);
        assert_eq!(memory.get_status_register_bit(SregBit::V), true);

        memory.update_sreg(128, 125, 126);
        assert_eq!(memory.get_status_register_bit(SregBit::V), true);
    }

    #[test]
    fn test_sreg_update_s_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg(0u8.wrapping_sub(120), 10, 0u8.wrapping_sub(130));
        assert_eq!(memory.get_status_register_bit(SregBit::S), true);

        memory.update_sreg(10, 20, 0u8.wrapping_sub(10));
        assert_eq!(memory.get_status_register_bit(SregBit::S), true);

        memory.update_sreg(20, 10, 10);
        assert_eq!(memory.get_status_register_bit(SregBit::S), false);
    }

    #[test]
    fn test_sreg_update_16bit_s_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg_16bit(0u16.wrapping_sub(120), 10, 0u16.wrapping_sub(130));
        assert_eq!(memory.get_status_register_bit(SregBit::S), true);

        memory.update_sreg_16bit(10, 20, 20);
        assert_eq!(memory.get_status_register_bit(SregBit::S), false);
    }

    #[test]
    fn test_sreg_update_16bit_v_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg_16bit(0, 0xffff, 0x8000);
        assert_eq!(memory.get_status_register_bit(SregBit::V), true);

        memory.update_sreg_16bit(0x8000, 0, 0x7fff);
        assert_eq!(memory.get_status_register_bit(SregBit::V), false);
    }

    #[test]
    fn test_sreg_update_16bit_n_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg_16bit(0, 0, 0x8000);
        assert_eq!(memory.get_status_register_bit(SregBit::N), true);

        memory.update_sreg_16bit(0, 0, 0x7fff);
        assert_eq!(memory.get_status_register_bit(SregBit::N), false);
    }

    #[test]
    fn test_sreg_update_16bit_z_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg_16bit(0xffff, 0xffff, 0);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), true);

        memory.update_sreg_16bit(0, 0, 1);
        assert_eq!(memory.get_status_register_bit(SregBit::Z), false);
    }

    #[test]
    fn test_sreg_update_16bit_c_bit() {
        let mut memory = Memory::new(100, vec![]).unwrap();

        memory.update_sreg_16bit(0, 0x7000, 0x8000);
        assert_eq!(memory.get_status_register_bit(SregBit::C), true);

        memory.update_sreg_16bit(0, 0x8000, 0x7000);
        assert_eq!(memory.get_status_register_bit(SregBit::C), false);
    }

    #[test]
    fn test_to_bit_position() {
        assert_eq!(Memory::to_bit_position(SregBit::I), 7);
        assert_eq!(Memory::to_bit_position(SregBit::T), 6);
        assert_eq!(Memory::to_bit_position(SregBit::H), 5);
        assert_eq!(Memory::to_bit_position(SregBit::S), 4);
        assert_eq!(Memory::to_bit_position(SregBit::V), 3);
        assert_eq!(Memory::to_bit_position(SregBit::N), 2);
        assert_eq!(Memory::to_bit_position(SregBit::Z), 1);
        assert_eq!(Memory::to_bit_position(SregBit::C), 0);
    }
}
