use crate::memory::{AddressSpace, Mmu};

use super::{
    byteutils::{get_word_from_bytes, split_word},
    flags::Flags,
    register::{Reg16, Reg8, Registers},
    Register,
};

pub struct Cpu<'a> {
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

    pub regs: Registers,
    pub flags: Flags,
    pub mmu: &'a mut Mmu,
}

#[allow(dead_code)]
impl<'a> Cpu<'a> {
    pub fn power_up(&mut self) {}

    pub fn new(mmu: &'a mut Mmu) -> Self {
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
            regs: Registers::default(),
            flags: Flags::new(),
            mmu,
        }
    }

    pub fn next_byte(&mut self) -> u8 {
        let b = self.mmu.get(self.regs.pc.value());
        self.regs.pc.inc();
        b
    }

    pub fn next_word(&mut self) -> u16 {
        let w = self.mmu.get_word(self.regs.pc.value());
        self.regs.pc.inc();
        self.regs.pc.inc();
        w
    }

    // Get RR
    fn get_af(&self) -> u16 {
        get_word_from_bytes(self.a.value(), self.f.value())
    }

    fn get_bc(&self) -> u16 {
        get_word_from_bytes(self.b.value(), self.c.value())
    }

    fn get_hl(&self) -> u16 {
        get_word_from_bytes(self.h.value(), self.l.value())
    }

    fn get_de(&self) -> u16 {
        get_word_from_bytes(self.d.value(), self.e.value())
    }

    // Set RR

    fn set_af(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.a.set(h);
        self.f.set(l);
    }

    fn set_bc(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.b.set(h);
        self.c.set(l);
    }

    fn set_hl(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.h.set(h);
        self.l.set(l);
    }

    fn set_de(&mut self, value: u16) {
        let (h, l) = split_word(value);
        self.d.set(h);
        self.e.set(l);
    }

    // Get & Set
    pub fn get_r8(&mut self, target_register: Reg8) -> u8 {
        match target_register {
            Reg8::A => self.a.value(),
            Reg8::F => self.f.value(),
            Reg8::B => self.b.value(),
            Reg8::C => self.c.value(),
            Reg8::D => self.d.value(),
            Reg8::E => self.e.value(),
            Reg8::H => self.h.value(),
            Reg8::L => self.l.value(),
        }
    }

    pub fn set_r8(&mut self, target_register: Reg8, value: u8) {
        match target_register {
            Reg8::A => self.a.set(value),
            Reg8::F => self.f.set(value),
            Reg8::B => self.b.set(value),
            Reg8::C => self.c.set(value),
            Reg8::D => self.d.set(value),
            Reg8::E => self.e.set(value),
            Reg8::H => self.h.set(value),
            Reg8::L => self.l.set(value),
        }
    }

    pub fn get_r16(&mut self, target_register: Reg16) -> u16 {
        match target_register {
            Reg16::AF => self.get_af(),
            Reg16::BC => self.get_bc(),
            Reg16::DE => self.get_de(),
            Reg16::HL => self.get_hl(),
            Reg16::SP => self.sp.value(),
            Reg16::PC => self.pc.value(),
        }
    }

    pub fn set_r16(&mut self, target_register: Reg16, value: u16) {
        match target_register {
            Reg16::AF => self.set_af(value),
            Reg16::BC => self.set_bc(value),
            Reg16::DE => self.set_de(value),
            Reg16::HL => self.set_hl(value),
            Reg16::SP => self.sp.set(value),
            Reg16::PC => self.pc.set(value),
        }
    }
}
