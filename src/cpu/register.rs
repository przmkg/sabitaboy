use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

const WORD_H: u16 = 0b11111111_00000000;
const WORD_L: u16 = 0b00000000_11111111;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Reg {
    A,
    F,
    AF,
    B,
    C,
    BC,
    D,
    E,
    DE,
    H,
    L,
    HL,
    SP,
    PC,
}

pub struct Register<T>
where
    T: Add,
{
    value: T,
}

impl<T> Register<T>
where
    T: Add + AddAssign + SubAssign + Copy,
{
    fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

impl Register<u8> {
    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn dec(&mut self) {
        self.value -= 1;
    }
}

impl Register<u16> {
    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn dec(&mut self) {
        self.value -= 1;
    }
}

impl Display for Register<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X}", self.value)
    }
}

impl Display for Register<u16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.value)
    }
}

#[allow(dead_code)]
pub struct Registers {
    pub a: Register<u8>,
    pub f: Register<u8>,
    pub b: Register<u8>,
    pub c: Register<u8>,
    pub d: Register<u8>,
    pub e: Register<u8>,
    pub h: Register<u8>,
    pub l: Register<u8>,
    pub sp: Register<u16>,
    pub pc: Register<u16>,
}

impl Registers {
    pub fn default() -> Self {
        Self {
            a: Register::new(0x01),
            f: Register::new(0xB0),
            b: Register::new(0x00),
            c: Register::new(0x13),
            d: Register::new(0x00),
            e: Register::new(0xD8),
            h: Register::new(0x01),
            l: Register::new(0x4D),
            sp: Register::new(0xFFFE),
            pc: Register::new(0x0100),
        }
    }
}

impl Registers {
    // Get

    // TODO Remove that method
    pub fn get_pc(&self) -> u16 {
        self.pc.value()
    }

    // Get RR
    pub fn get_hl(&self) -> u16 {
        get_word_from_bytes(self.h.value, self.l.value)
    }

    pub fn get_bc(&self) -> u16 {
        get_word_from_bytes(self.b.value, self.c.value)
    }

    pub fn get_de(&self) -> u16 {
        get_word_from_bytes(self.d.value, self.e.value)
    }

    // Set RR

    pub fn set_pc(&mut self, address: u16) {
        self.pc.set(address);
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp.set(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.h.set(h);
        self.l.set(l);
    }

    pub fn set_bc(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.b.set(h);
        self.c.set(l);
    }

    pub fn set_de(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.d.set(h);
        self.e.set(l);
    }
}

fn split_word(word: u16) -> (u8, u8) {
    let h = (word >> 8) as u8;
    let l = word as u8;

    (h, l)
}

fn get_word_from_bytes(h: u8, l: u8) -> u16 {
    (h as u16) << 8 & l as u16
}

fn set_lower_byte(word: u16, value: u8) -> u16 {
    (word & WORD_H) | value as u16
}

fn set_higher_byte(word: u16, value: u8) -> u16 {
    (word & WORD_L) | ((value as u16) << 8)
}

#[cfg(test)]
mod tests {
    use crate::cpu::register::{set_higher_byte, set_lower_byte};

    #[test]
    pub fn set_single_register() {
        let a = 0b10110101_00101110;
        let b = 0b11001001;

        assert_eq!(0b10110101_11001001, set_lower_byte(a, b));
        assert_eq!(0b11001001_00101110, set_higher_byte(a, b));
    }
}
