use std::{cell::RefCell, rc::Rc};

use crate::{cpu::Cpu, memory::AddressSpace};

use super::register::{Reg16, Reg8, Register};

type Cycles = u8;

pub enum HLAction {
    Inc,
    Dec,
    None,
}

impl<'a> Cpu<'a> {
    // JP a16, 3, 16
    fn jp_a16(&mut self) -> Cycles {
        let next = self.next_word();
        self.regs.pc.set(next);

        16
    }

    // XOR A, 1, 4
    // Z
    fn xor_a(&mut self) -> Cycles {
        let a = self.regs.a.value();

        if a ^ a == 0 {
            self.flags.zero = true;
        }

        4
    }

    // LD RR, d16, 3, 12
    fn ld_rr_d16(&mut self, target_register: Reg16) -> Cycles {
        let value = self.next_word();
        self.set_r16(target_register, value);

        12
    }

    // LD R, d8, 2, 8
    fn ld_r_d8(&mut self, target_register: Reg8) -> Cycles {
        let value = self.next_byte();
        self.set_r8(target_register, value);

        8
    }

    // LD (HL), R, 1, 8
    fn ld_hl_r(&mut self, value: u8, action: HLAction) -> Cycles {
        let hl_value = self.get_r16(Reg16::HL);
        self.mmu.set(hl_value, value);

        match action {
            HLAction::Inc => self.set_r16(Reg16::HL, hl_value.saturating_add(1)),
            HLAction::Dec => self.set_r16(Reg16::HL, hl_value.saturating_sub(1)),
            HLAction::None => {}
        }

        8
    }

    // LD (RR), R, 1, 8
    fn ld_a16_r(&mut self, target_address: u16, value: u8) -> Cycles {
        self.mmu.set(target_address, value);

        8
    }

    // DEC R, 1, 4
    // Z 1 H
    fn dec_r(&mut self, target_register: Reg8) -> Cycles {
        // TODO impl
        // let value = target_register.value();

        // target_register.dec();

        4
    }

    pub fn execute(&mut self) -> u8 {
        let opcode = self.next_byte();
        println!("Opcode: {:#04X}", opcode);

        match opcode {
            // NOP
            0x00 => 4,
            0xC3 => self.jp_a16(),
            0xAF => self.xor_a(),
            // LD RR, d16
            0x01 => self.ld_rr_d16(Reg16::BC),
            0x11 => self.ld_rr_d16(Reg16::DE),
            0x21 => self.ld_rr_d16(Reg16::HL),
            0x31 => self.ld_rr_d16(Reg16::SP),
            0x05 => {
                // TODO Implement that
                // self.dec_r(&mut self.a)
                0
            }
            // LD R, d8
            0x06 => self.ld_r_d8(Reg8::B),
            0x16 => self.ld_r_d8(Reg8::D),
            0x26 => self.ld_r_d8(Reg8::H),
            0x0E => self.ld_r_d8(Reg8::C),
            0x1E => self.ld_r_d8(Reg8::E),
            0x2E => self.ld_r_d8(Reg8::L),
            0x3E => self.ld_r_d8(Reg8::A),
            // LD (RR), R
            0x02 => {
                let (target_address, value) = (self.get_r16(Reg16::BC), self.a.value());
                self.ld_a16_r(target_address, value)
            }
            0x12 => {
                let (target_address, value) = (self.get_r16(Reg16::DE), self.a.value());
                self.ld_a16_r(target_address, value)
            }
            0x22 => {
                let value = self.regs.a.value();
                self.ld_hl_r(value, HLAction::Inc)
            }
            0x32 => {
                let value = self.regs.a.value();
                self.ld_hl_r(value, HLAction::Dec)
            }
            _ => {
                panic!("Unimplemented: {:#04X}", opcode);
            }
        }

        // Check for interrupts
    }
}
