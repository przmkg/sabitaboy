use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, SubAssign},
    rc::Rc,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub struct Register<T> {
    value: T,
}

impl<T> Register<T>
where
    T: Add + AddAssign + SubAssign + Copy,
{
    pub fn new(value: T) -> Self {
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

impl<T> Deref for Register<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Register<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
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
    pub a: Rc<Register<u8>>,
    pub f: Rc<Register<u8>>,
    pub b: Rc<Register<u8>>,
    pub c: Rc<Register<u8>>,
    pub d: Rc<Register<u8>>,
    pub e: Rc<Register<u8>>,
    pub h: Rc<Register<u8>>,
    pub l: Rc<Register<u8>>,
    pub sp: Register<u16>,
    pub pc: Register<u16>,
}

impl Registers {
    pub fn default() -> Self {
        Self {
            a: Rc::new(Register::new(0x01)),
            f: Rc::new(Register::new(0xB0)),
            b: Rc::new(Register::new(0x00)),
            c: Rc::new(Register::new(0x13)),
            d: Rc::new(Register::new(0x00)),
            e: Rc::new(Register::new(0xD8)),
            h: Rc::new(Register::new(0x01)),
            l: Rc::new(Register::new(0x4D)),
            sp: Register::new(0xFFFE),
            pc: Register::new(0x0100),
        }
    }
}
