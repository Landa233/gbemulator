use std::fmt;

pub enum Flag {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub pc: u16,
    pub sp: u16,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AF: 0x{:02X}{:02X}, BC: 0x{:02X}{:02X}, DE: 0x{:02X}{:02X}, HL: 0x{:02X}{:02X}, PC: {}, SP: 0x{:02X}",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp
        )
    }
}

impl Registers {
    pub fn new() -> Registers {
        //Set initial values according to pandocs
        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            f: 0xB0,
            pc: 0x100,
            sp: 0xFFFE,
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.f |= flag as u8
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.f &= flag as u8 ^ 0xFF
    }

    pub fn clear_all_flags(&mut self) {
        self.clear_flag(Flag::Z);
        self.clear_flag(Flag::C);
        self.clear_flag(Flag::H);
        self.clear_flag(Flag::N);
    }

    pub fn check_flag(&self, flag: Flag) -> bool {
        let flag_value = flag as u8;
        self.f & flag_value == flag_value as u8
    }
}
