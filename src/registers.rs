#[derive(Debug, PartialEq)]
pub struct Registers {
    pub r: [u8; 32],
    pub io: [u8; 64],
    pub stack: [u8; 66000],
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
        ((self.r[27] as u16) << 8) | self.r[26] as u16
    }
    pub fn set_x(&mut self, new_x: u16) {
        self.r[27] = (new_x >> 8) as u8;
        self.r[26] = (new_x & 0x00ff) as u8;
    }

    pub fn y(&self) -> u16 {
        ((self.io[29] as u16) << 8) | self.io[28] as u16
    }
    pub fn set_y(&mut self, new_y: u16) {
        self.r[29] = (new_y >> 8) as u8;
        self.r[28] = (new_y & 0x00ff) as u8;
    }

    pub fn z(&self) -> u16 {
        ((self.r[31] as u16) << 8) | self.r[30] as u16
    }
    pub fn set_z(&mut self, new_z: u16) {
        self.r[31] = (new_z >> 8) as u8;
        self.r[30] = (new_z & 0x00ff) as u8;
    }
}
