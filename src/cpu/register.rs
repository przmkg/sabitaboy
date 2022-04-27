use std::fmt::Display;

const WORD_H: u16 = 0b11111111_00000000;
const WORD_L: u16 = 0b00000000_11111111;

pub struct Register {
    value: u16,
}

impl Register {
    fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.value)
    }
}

#[allow(dead_code)]
pub struct Registers {
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: Register,
    pub pc: Register,
}

impl Registers {
    pub fn default() -> Self {
        Self {
            af: Register::new(0x01B0),
            bc: Register::new(0x0013),
            de: Register::new(0x00D8),
            hl: Register::new(0x014D),
            sp: Register::new(0xFFFE),
            pc: Register::new(0x0100),
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
    af: {},
    bc: {},
    de: {},
    hl: {},
    sp: {},
    pc: {} 
        "#,
            self.af, self.bc, self.de, self.hl, self.sp, self.pc
        )
    }
}

impl Registers {
    // Get

    pub fn get_pc(&self) -> u16 {
        self.pc.value()
    }

    pub fn get_a(&self) -> u8 {
        (self.af.value() >> 8) as u8
    }

    // Set RR

    pub fn set_pc(&mut self, address: u16) {
        self.pc.set(address);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.hl.set(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.bc.set(value);
    }

    pub fn set_de(&mut self, value: u16) {
        self.de.set(value);
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp.set(value);
    }

    // Set R

    pub fn set_c(&mut self, value: u8) {
        self.bc.set(set_lower_byte(self.bc.value(), value));
    }

    pub fn set_b(&mut self, value: u8) {
        self.bc.set(set_higher_byte(self.bc.value(), value));
    }
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
