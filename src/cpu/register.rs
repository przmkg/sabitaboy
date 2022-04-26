use std::fmt::Display;

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
