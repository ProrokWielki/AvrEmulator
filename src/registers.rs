#[derive(Debug, PartialEq, Clone)]
pub struct Registers {
    pub r: [u8; 32],
    pub io: [u8; 64],
    pub stack: [u8; 66000], //TODO: this should be changed
    pub pc: i32,
    pub sreg_i: bool,
    pub sreg_t: bool,
    pub sreg_h: bool,
    pub sreg_s: bool,
    pub sreg_v: bool,
    pub sreg_n: bool,
    pub sreg_z: bool,
    pub sreg_c: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r: [0; 32],
            io: [0; 64],
            pc: 0,
            stack: [0; 66000],
            sreg_i: false,
            sreg_t: false,
            sreg_h: false,
            sreg_s: false,
            sreg_v: false,
            sreg_n: false,
            sreg_z: false,
            sreg_c: false,
        }
    }

    pub fn print_sreg(&self) {
        println!(
            "i t h s v n z c\n{} {} {} {} {} {} {} {}",
            self.sreg_i as i32,
            self.sreg_t as i32,
            self.sreg_h as i32,
            self.sreg_s as i32,
            self.sreg_v as i32,
            self.sreg_n as i32,
            self.sreg_z as i32,
            self.sreg_c as i32
        );
    }

    pub fn print_r(&self) {
        for i in 0..32 {
            println!("r{}: {}", i, self.r[i] as i32)
        }
    }

    pub fn as_16bit(&self, r: usize) -> u16 {
        ((self.r[r + 1] as u16) << 8) | self.r[r] as u16
    }
    pub fn set_as_16bit(&mut self, r: usize, new_value: u16) {
        self.r[r + 1] = (new_value >> 8) as u8;
        self.r[r] = (new_value & 0x00ff) as u8;
    }

    pub fn sp(&self) -> u16 {
        ((self.io[62] as u16) << 8) | self.io[61] as u16
    }
    pub fn set_sp(&mut self, new_sp: u16) {
        self.io[62] = (new_sp >> 8) as u8;
        self.io[61] = (new_sp & 0x00ff) as u8;
    }

    pub fn x(&self) -> u16 {
        self.as_16bit(26)
    }
    pub fn set_x(&mut self, new_x: u16) {
        self.set_as_16bit(26, new_x);
    }

    pub fn y(&self) -> u16 {
        self.as_16bit(28)
    }
    pub fn set_y(&mut self, new_y: u16) {
        self.set_as_16bit(28, new_y);
    }

    pub fn z(&self) -> u16 {
        self.as_16bit(30)
    }
    pub fn set_z(&mut self, new_z: u16) {
        self.set_as_16bit(30, new_z);
    }

    pub fn update_sreg(&mut self, lhs: u8, rhs: u8, result: u8) {
        self.sreg_h = (!Self::bit8(lhs, 3) && Self::bit8(rhs, 3))
            || (Self::bit8(rhs, 3) && Self::bit8(result, 3))
            || (Self::bit8(result, 3) && !Self::bit8(lhs, 3));

        self.sreg_v = (Self::bit8(lhs, 7) && !Self::bit8(rhs, 7) && !Self::bit8(rhs, 7))
            || (!Self::bit8(lhs, 7) && Self::bit8(rhs, 7) && Self::bit8(result, 7));

        self.sreg_n = Self::bit8(result, 7);

        self.sreg_z = result == 0;

        self.sreg_c = (!Self::bit8(lhs, 7) && Self::bit8(rhs, 7))
            || (Self::bit8(rhs, 7) && Self::bit8(result, 7))
            || (Self::bit8(result, 7) && !Self::bit8(lhs, 7));

        self.sreg_s = self.sreg_n != self.sreg_v;
    }

    pub fn update_sreg_16bit(&mut self, lhs: u16, _rhs: u16, result: u16) {
        self.sreg_v = Self::bit16(result, 15) && !Self::bit16(lhs, 15);

        self.sreg_n = Self::bit16(result, 15);

        self.sreg_z = result == 0;

        self.sreg_c = Self::bit16(result, 15) && !Self::bit16(lhs, 15);

        self.sreg_s = self.sreg_n != self.sreg_v;
    }

    pub fn update_sreg_keep_z_if_resoult_zero(&mut self, lhs: u8, rhs: u8, result: u8) {
        let old_z = self.sreg_z;

        self.update_sreg(lhs, rhs, result);

        if result == 0 {
            self.sreg_z = old_z;
        } else {
            self.sreg_z = false;
        }
    }

    fn bit16(var: u16, bit: u16) -> bool {
        (var & (1 << bit)) != 0
    }

    fn bit8(var: u8, bit: u8) -> bool {
        Self::bit16(var as u16, bit as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit8() {
        let test_value = 0b1011_0010;

        for i in 0..7 {
            assert_eq!(Registers::bit8(test_value, i), (test_value & (1 << i)) != 0);
        }
    }

    #[test]
    fn test_bit16() {
        let test_value = 0xabcd;

        for i in 0..15 {
            assert_eq!(
                Registers::bit16(test_value, i),
                (test_value & (1 << i)) != 0
            );
        }
    }

    #[test]
    fn test_as_16bit() {
        let msb: u8 = 0xf0;
        let lsb: u8 = 0x5a;
        let register = 12;

        let mut registers = Registers::new();
        registers.r[register + 1] = msb;
        registers.r[register] = lsb;

        assert_eq!(
            registers.as_16bit(register),
            ((msb as u16) << 8) | lsb as u16
        );
    }

    #[test]
    fn test_set_as_16bit() {
        let data: u16 = 0xf05a;
        let register = 12;

        let mut registers = Registers::new();
        registers.set_as_16bit(register, data);

        assert_eq!(registers.r[register], (data & 0x00ff) as u8);
        assert_eq!(registers.r[register + 1], (data >> 8) as u8);
    }

    #[test]
    fn test_sp() {
        let lsb: u8 = 0xa5;
        let msb: u8 = 0xfa;

        let mut registers = Registers::new();
        registers.io[61] = lsb;
        registers.io[62] = msb;

        assert_eq!(registers.sp(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_sp() {
        let new_sp = 0x8f12;

        let mut registers = Registers::new();
        registers.set_sp(new_sp);

        assert_eq!(registers.sp(), new_sp);
    }

    #[test]
    fn test_x() {
        let lsb: u8 = 0x18;
        let msb: u8 = 0x81;

        let mut registers = Registers::new();
        registers.r[26] = lsb;
        registers.r[27] = msb;

        assert_eq!(registers.x(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_x() {
        let new_x = 0xf0f0;

        let mut registers = Registers::new();
        registers.set_x(new_x);

        assert_eq!(registers.x(), new_x);
    }

    #[test]
    fn test_y() {
        let lsb: u8 = 0x12;
        let msb: u8 = 0x34;

        let mut registers = Registers::new();
        registers.r[28] = lsb;
        registers.r[29] = msb;

        assert_eq!(registers.y(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_y() {
        let new_y = 0xdead;

        let mut registers = Registers::new();
        registers.set_y(new_y);

        assert_eq!(registers.y(), new_y);
    }

    #[test]
    fn test_z() {
        let lsb: u8 = 0x43;
        let msb: u8 = 0x21;

        let mut registers = Registers::new();
        registers.r[30] = lsb;
        registers.r[31] = msb;

        assert_eq!(registers.z(), ((msb as u16) << 8) | (lsb as u16));
    }

    #[test]
    fn test_set_z() {
        let new_z = 0xbeef;

        let mut registers = Registers::new();
        registers.set_y(new_z);

        assert_eq!(registers.y(), new_z);
    }

    #[test]
    fn test_sreg_update_borrow_from_bit_3() {
        let mut registers = Registers::new();

        registers.update_sreg(8, 2, 3);
        assert_eq!(registers.sreg_h, false);

        registers.update_sreg(2, 9, 2);
        assert_eq!(registers.sreg_h, true);

        registers.update_sreg(10, 11, 12);
        assert_eq!(registers.sreg_h, true);

        registers.update_sreg(3, 2, 8);
        assert_eq!(registers.sreg_h, true);
    }

    #[test]
    fn test_sreg_update_result_zero_no_keep() {
        let mut registers = Registers::new();

        registers.update_sreg(1, 2, 3);
        assert_eq!(registers.sreg_z, false);

        registers.update_sreg(2, 1, 0);
        assert_eq!(registers.sreg_z, true);
    }

    #[test]
    fn test_sreg_update_result_zero_keep() {
        let mut registers = Registers::new();

        registers.sreg_z = true;
        registers.update_sreg_keep_z_if_resoult_zero(1, 2, 3);
        assert_eq!(registers.sreg_z, false);

        registers.sreg_z = true;
        registers.update_sreg_keep_z_if_resoult_zero(1, 2, 0);
        assert_eq!(registers.sreg_z, true);

        registers.sreg_z = false;
        registers.update_sreg_keep_z_if_resoult_zero(1, 2, 3);
        assert_eq!(registers.sreg_z, false);

        registers.sreg_z = false;
        registers.update_sreg_keep_z_if_resoult_zero(2, 1, 0);
        assert_eq!(registers.sreg_z, false);
    }

    #[test]
    fn test_sreg_update_carry_bit() {
        let mut registers = Registers::new();

        registers.update_sreg(128, 2, 3);
        assert_eq!(registers.sreg_c, false);

        registers.update_sreg(2, 128, 0);
        assert_eq!(registers.sreg_c, true);

        registers.update_sreg(128, 129, 130);
        assert_eq!(registers.sreg_c, true);

        registers.update_sreg(2, 1, 131);
        assert_eq!(registers.sreg_c, true);
    }

    #[test]
    fn test_sreg_update_n_bit() {
        let mut registers = Registers::new();

        registers.update_sreg(0, 0, 127);
        assert_eq!(registers.sreg_c, false);

        registers.update_sreg(0, 0, 128);
        assert_eq!(registers.sreg_c, true);
    }

    #[test]
    fn test_sreg_update_v_bit() {
        let mut registers = Registers::new();

        registers.update_sreg(0, 1, 2);
        assert_eq!(registers.sreg_v, false);

        registers.update_sreg(127, 128, 129);
        assert_eq!(registers.sreg_v, true);

        registers.update_sreg(128, 125, 126);
        assert_eq!(registers.sreg_v, true);
    }

    #[test]
    fn test_sreg_update_s_bit() {
        let mut registers = Registers::new();

        registers.update_sreg(0u8.wrapping_sub(120), 10, 0u8.wrapping_sub(130));
        assert_eq!(registers.sreg_s, true);

        registers.update_sreg(10, 20, 0u8.wrapping_sub(10));
        assert_eq!(registers.sreg_s, true);

        registers.update_sreg(20, 10, 10);
        assert_eq!(registers.sreg_s, false);
    }

    #[test]
    fn test_sreg_update_16bit_s_bit() {
        let mut registers = Registers::new();

        registers.update_sreg_16bit(0u16.wrapping_sub(120), 10, 0u16.wrapping_sub(130));
        assert_eq!(registers.sreg_s, true);

        registers.update_sreg_16bit(10, 20, 20);
        assert_eq!(registers.sreg_s, false);
    }

    #[test]
    fn test_sreg_update_16bit_v_bit() {
        let mut registers = Registers::new();

        registers.update_sreg_16bit(0, 0xffff, 0x8000);
        assert_eq!(registers.sreg_v, true);

        registers.update_sreg_16bit(0x8000, 0, 0x7fff);
        assert_eq!(registers.sreg_v, false);
    }

    #[test]
    fn test_sreg_update_16bit_n_bit() {
        let mut registers = Registers::new();

        registers.update_sreg_16bit(0, 0, 0x8000);
        assert_eq!(registers.sreg_n, true);

        registers.update_sreg_16bit(0, 0, 0x7fff);
        assert_eq!(registers.sreg_n, false);
    }

    #[test]
    fn test_sreg_update_16bit_z_bit() {
        let mut registers = Registers::new();

        registers.update_sreg_16bit(0xffff, 0xffff, 0);
        assert_eq!(registers.sreg_z, true);

        registers.update_sreg_16bit(0, 0, 1);
        assert_eq!(registers.sreg_z, false);
    }

    #[test]
    fn test_sreg_update_16bit_c_bit() {
        let mut registers = Registers::new();

        registers.update_sreg_16bit(0, 0x7000, 0x8000);
        assert_eq!(registers.sreg_c, true);

        registers.update_sreg_16bit(0, 0x8000, 0x7000);
        assert_eq!(registers.sreg_c, false);
    }
}
