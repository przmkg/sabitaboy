use crate::memory::{AddressSpace, Mmu};

use super::{flags::Flags, op};

#[allow(dead_code)]
struct Regs {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

// TODO Remove when not needed anymore
pub struct Cpu<'a> {
    regs: Regs,
    flags: Flags,
    mmu: &'a mut Mmu,
}

#[allow(dead_code)]
impl<'a> Cpu<'a> {
    pub fn power_up(&mut self) {}

    pub fn new(mmu: &'a mut Mmu) -> Self {
        Self {
            regs: Regs {
                af: 0x01B0,
                bc: 0x0013,
                de: 0x00D8,
                hl: 0x014D,
                sp: 0xFFFE,
                pc: 0x0100,
            },
            flags: Flags::new(),
            mmu,
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.next_byte();
        op::execute(opcode, self);
    }

    pub fn next_byte(&mut self) -> u8 {
        let b = self.mmu.get(self.regs.pc);
        self.regs.pc += 1;
        b
    }

    pub fn next_word(&mut self) -> u16 {
        let w = self.mmu.get_word(self.regs.pc);
        self.regs.pc += 2;
        w
    }

    pub fn get_a(&self) -> u8 {
        (self.regs.af >> 8) as u8
    }

    pub fn get_pc(&self) -> u16 {
        self.regs.pc
    }

    pub fn set_pc(&mut self, address: u16) {
        self.regs.pc = address;
    }

    pub fn set_fz(&mut self, value: bool) {
        self.flags.zero = value;
    }
}
