use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, SubAssign},
    rc::Rc,
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
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
    pub fn inc(&mut self) -> (u8, bool) {
        let (result, overflow) = self.value.overflowing_add(1);
        self.value = result;

        (result, overflow)
    }

    pub fn dec(&mut self) -> (u8, bool) {
        let (result, overflow) = self.value.overflowing_sub(1);
        self.value = result;

        (result, overflow)
    }
}

impl Register<u16> {
    pub fn inc(&mut self) -> (u16, bool) {
        let (result, overflow) = self.value.overflowing_add(1);
        self.value = result;

        (result, overflow)
    }

    pub fn dec(&mut self) -> (u16, bool) {
        let (result, overflow) = self.value.overflowing_sub(1);
        self.value = result;

        (result, overflow)
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
