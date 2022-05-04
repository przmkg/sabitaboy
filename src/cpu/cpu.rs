use crate::memory::{AddressSpace, Mmu};

use super::{flags::Flags, op, register::Registers};

// TODO Remove when not needed anymore
pub struct Cpu<'a> {
    regs: Registers,
    flags: Flags,
    mmu: &'a mut Mmu,
}

#[allow(dead_code)]
impl<'a> Cpu<'a> {
    pub fn power_up(&mut self) {}

    pub fn new(mmu: &'a mut Mmu) -> Self {
        Self {
            regs: Registers::default(),
            flags: Flags::new(),
            mmu,
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.next_byte();
        op::execute(opcode, self);
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

    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    pub fn flags_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }

    pub fn regs(&self) -> &Registers {
        &self.regs
    }

    pub fn regs_mut(&mut self) -> &mut Registers {
        &mut self.regs
    }
}
