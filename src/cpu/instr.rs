use crate::{cpu::Cpu, memory::AddressSpace};

use super::register::{Reg16, Reg8};

type Cycles = u8;

enum RegisterAction {
    Inc,
    Dec,
    None,
}

pub enum Target {
    Address8(u8),
    Address16(u16),
    Register8(Reg8),
    Register16(Reg16),
    RegisterAddress16(Reg16),
}

impl<'a> Cpu<'a> {
    // JP a16, 3, 16
    fn jp_a16(&mut self, condition: bool) -> Cycles {
        let next = self.read_word();

        if condition {
            self.pc.set(next);
            16
        } else {
            12
        }
    }

    // JR cc, r8, 2, 12/8
    fn jr_r8(&mut self, condition: bool) -> Cycles {
        let value = self.read_byte() as i8;

        if condition {
            // Mixed integer ops are not yet on Rust stable
            let address = if value.is_positive() {
                self.pc.value().wrapping_add(value as u16)
            } else {
                self.pc.value().wrapping_sub(value.unsigned_abs() as u16)
            };

            self.pc.set(address);

            12
        } else {
            8
        }
    }

    // XOR R, 1, 4
    // Z 0 0 0
    fn xor_r(&mut self, target_register: Reg8) -> Cycles {
        let reg = self.get_r(target_register);

        reg.set(reg.value() ^ reg.value());

        if reg.value() == 0 {
            self.flags.zero = true;
        }

        self.flags.carry = false;
        self.flags.half_carry = false;
        self.flags.sub = false;

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
        let half_carry = is_sub_half_carry(reg.value(), 1);
        let (result, _) = reg.dec();

        self.flags.sub = true;
        self.flags.zero = result == 0;
        self.flags.half_carry = half_carry;

        4
    }

    // LDH (a8), A, 2, 12
    fn ldh_a_a8(&mut self, target: Target, value: u8) -> Cycles {
        match target {
            Target::Address8(address) => self.mmu.set(0xFF00 + address as u16, value),
            Target::Register8(reg) => {
                let value = self.mmu.get(0xFF00 + value as u16);
                self.get_r(reg).set(value);
            }
            _ => panic!("Not possible"),
        }

        12
    }

    // LD (C), A, 2, 8
    fn ld_c_a(&mut self, target: Target, value: u8) -> Cycles {
        match target {
            Target::Address8(address) => self.mmu.set(0xFF00 + address as u16, value),
            Target::Register8(reg) => {
                let value = self.mmu.get(0xFF00 + value as u16);
                self.get_r(reg).set(value);
            }
            _ => panic!("Not possible"),
        }
        8
    }

    // CP d8, 2, 8
    // Z 1 H C
    fn cp_d8(&mut self, value: u8) -> Cycles {
        let (result, overflow) = self.a.value().overflowing_sub(value);

        self.flags.zero = result == 0;
        self.flags.sub = true;
        self.flags.carry = overflow;
        // TODO Check if half carry is correct
        self.flags.half_carry = is_sub_half_carry(self.a.value(), value);

        8
    }

    pub fn decode(&mut self, opcode: u8) -> Cycles {
        match opcode {
            // NOP
            0x00 => 4,
            0xCB => {
                let cb_opcode = self.read_byte();
                self.match_cb_prefix(cb_opcode)
            }
            // TODO Impl DI & EI
            0xF3 => 4,
            0xFB => 4,
            // JP cc, a16
            0xC2 => self.jp_a16(!self.flags.zero),
            0xD2 => self.jp_a16(!self.flags.carry),
            0xC3 => self.jp_a16(true),
            0xCA => self.jp_a16(self.flags.zero),
            0xDA => self.jp_a16(self.flags.carry),
            // JR cc, r8
            0x20 => self.jr_r8(!self.flags.zero),
            0x30 => self.jr_r8(!self.flags.carry),
            0x18 => self.jr_r8(true),
            0x28 => self.jr_r8(self.flags.zero),
            0x38 => self.jr_r8(self.flags.carry),
            //
            0xAF => self.xor_r(Reg8::A),
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
            // LDH (a8), A / LDH A, (a8)
            0xE0 => {
                let address = self.read_byte();
                let value = self.a.value();
                self.ldh_a_a8(Target::Address8(address), value)
            }
            0xF0 => {
                let value = self.read_byte();
                self.ldh_a_a8(Target::Register8(Reg8::A), value)
            }
            // LD (C), A / LD A, (C)
            0xE2 => {
                let address = self.get_r(Reg8::C).value();
                let value = self.a.value();
                self.ld_c_a(Target::Address8(address), value)
            }
            0xF2 => {
                let value = self.get_r(Reg8::C).value();
                self.ld_c_a(Target::Register8(Reg8::A), value)
            }
            // CP
            0xFE => {
                let value = self.read_byte();
                self.cp_d8(value)
            }
            _ => {
                panic!("Unimplemented: {:#04X}", opcode);
            }
        }

        // Check for interrupts
    }
}

fn is_sub_half_carry(a: u8, b: u8) -> bool {
    ((a & 0xF).wrapping_sub(b) & 0x10) != 0
}
