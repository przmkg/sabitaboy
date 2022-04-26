use crate::memory::{AddressSpace, Mmu};

use super::{flags::Flags, op, register::Registers, ExecutionResult};

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

    pub fn print_registers(&self) {
        println!("{}", self.regs);
    }

    pub fn execute(&mut self) -> ExecutionResult {
        let opcode = self.next_byte();
        op::execute(opcode, self)
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

    pub fn get_a(&self) -> u8 {
        (self.regs.af.value() >> 8) as u8
    }

    pub fn get_pc(&self) -> u16 {
        self.regs.pc.value()
    }

    pub fn set_pc(&mut self, address: u16) {
        self.regs.pc.set(address);
    }

    pub fn set_fz(&mut self, value: bool) {
        self.flags.set_zero(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.regs.hl.set(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.regs.bc.set(value);
    }

    pub fn set_de(&mut self, value: u16) {
        self.regs.de.set(value);
    }

    pub fn set_sp(&mut self, value: u16) {
        self.regs.sp.set(value);
    }
}
