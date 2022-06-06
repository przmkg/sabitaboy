use crate::{cpu::Cpu, memory::AddressSpace};

use super::register::{Reg16, Reg8};

type Cycles = u8;

enum RegisterAction {
    Inc,
    Dec,
    None,
}

impl<'a> Cpu<'a> {
    // JP a16, 3, 16
    fn jp_a16(&mut self, condition: bool) -> Cycles {
        if condition {
            let next = self.read_word();
            self.pc.set(next);

            16
        } else {
            12
        }
    }

    // JR cc, d8, 2, 12/8
    fn jr_d8(&mut self, condition: bool) -> Cycles {
        if condition {
            let address = self.pc.value().wrapping_add(self.peek_byte() as u16);
            self.pc.set(address);

            12
        } else {
            8
        }
    }

    // XOR R, 1, 4
    // Z
    fn xor_r(&mut self, target_register: Reg8) -> Cycles {
        let reg = self.get_r(target_register);

        reg.set(reg.value() ^ reg.value());

        if reg.value() == 0 {
            self.flags.zero = true;
        }

        4
    }

    // LD RR, d16, 3, 12
    fn ld_rr_d16(&mut self, target_register: Reg16) -> Cycles {
        let value = self.read_word();
        self.set_r16(target_register, value);

        12
    }

    // LD R, d8, 2, 8
    fn ld_r_d8(&mut self, target_register: Reg8) -> Cycles {
        let value = self.read_byte();
        self.get_r(target_register).set(value);

        8
    }

    // LD (RR), R, 1, 8
    fn ld_a16_r(&mut self, address_register: Reg16, value: u8, action: RegisterAction) -> Cycles {
        let register_value = self.get_r16(address_register);
        self.mmu.set(register_value, value);

        match action {
            RegisterAction::Inc => {
                self.set_r16(address_register, register_value.overflowing_sub(1).0)
            }
            RegisterAction::Dec => {
                self.set_r16(address_register, register_value.overflowing_sub(1).0)
            }
            RegisterAction::None => {}
        }

        8
    }

    // DEC R, 1, 4
    // Z 1 H
    fn dec_r(&mut self, target_register: Reg8) -> Cycles {
        let reg = self.get_r(target_register);
        let half_carry = ((reg.value() & 0xF).wrapping_sub(1) & 0x10) != 0;
        let (result, _) = reg.dec();

        self.flags.sub = true;
        self.flags.zero = result == 0;
        self.flags.half_carry = half_carry;

        4
    }

    pub fn execute(&mut self) -> u8 {
        println!("PC: {:04X}", self.pc.value());
        let opcode = self.read_byte();
        println!("Opcode: {:#04X}", opcode);

        match opcode {
            // NOP
            0x00 => 4,
            0xC3 => self.jp_a16(true),
            0xAF => self.xor_r(Reg8::A),
            // JR cc, d8
            0x20 => self.jr_d8(!self.flags.zero),
            0x30 => self.jr_d8(!self.flags.carry),
            // LD RR, d16
            0x01 => self.ld_rr_d16(Reg16::BC),
            0x11 => self.ld_rr_d16(Reg16::DE),
            0x21 => self.ld_rr_d16(Reg16::HL),
            0x31 => self.ld_rr_d16(Reg16::SP),
            // DEC R
            0x05 => self.dec_r(Reg8::B),
            0x15 => self.dec_r(Reg8::D),
            0x25 => self.dec_r(Reg8::H),
            0x0D => self.dec_r(Reg8::C),
            0x1D => self.dec_r(Reg8::E),
            0x2D => self.dec_r(Reg8::L),
            0x3D => self.dec_r(Reg8::A),
            // LD R, d8
            0x06 => self.ld_r_d8(Reg8::B),
            0x16 => self.ld_r_d8(Reg8::D),
            0x26 => self.ld_r_d8(Reg8::H),
            0x0E => self.ld_r_d8(Reg8::C),
            0x1E => self.ld_r_d8(Reg8::E),
            0x2E => self.ld_r_d8(Reg8::L),
            0x3E => self.ld_r_d8(Reg8::A),
            // LD (RR), R
            0x02 => self.ld_a16_r(Reg16::BC, self.a.value(), RegisterAction::None),
            0x12 => self.ld_a16_r(Reg16::DE, self.a.value(), RegisterAction::None),
            0x22 => self.ld_a16_r(Reg16::HL, self.a.value(), RegisterAction::Inc),
            0x32 => self.ld_a16_r(Reg16::HL, self.a.value(), RegisterAction::Dec),
            0x71 => self.ld_a16_r(Reg16::HL, self.c.value(), RegisterAction::None),
            _ => {
                panic!("Unimplemented: {:#04X}", opcode);
            }
        }

        // Check for interrupts
    }
}
