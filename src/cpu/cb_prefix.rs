use crate::{cpu::register::Reg16, memory::AddressSpace};

use super::{register::Reg8, Cpu};

enum Target {
    Register(Reg8),
    RegisterAddress(Reg16),
}

const TARGETS: [Target; 8] = [
    Target::Register(Reg8::B),
    Target::Register(Reg8::C),
    Target::Register(Reg8::D),
    Target::Register(Reg8::E),
    Target::Register(Reg8::H),
    Target::Register(Reg8::L),
    Target::RegisterAddress(Reg16::HL),
    Target::Register(Reg8::A),
];

type Cycles = u8;

impl<'a> Cpu<'a> {
    // BIT n, R, 2, 8
    // Z 0 1
    fn bit_n_r(&mut self, n: u8, value: u8, is_hl: bool) -> Cycles {
        self.flags.zero = value & n != 0;
        self.flags.sub = false;
        self.flags.half_carry = true;

        if is_hl {
            16
        } else {
            8
        }
    }

    // TODO Implement other CB instructions

    pub fn match_cb_prefix(&mut self, opcode: u8) -> Cycles {
        println!("CB: {:02X}", opcode);

        let mut is_hl = false;
        let target = &TARGETS[opcode as usize % 8];

        let target_value = match target {
            Target::Register(r8) => self.get_r(*r8).value(),
            Target::RegisterAddress(r16) => {
                is_hl = true;
                let address = self.get_r16(*r16);
                self.mmu.get(address)
            }
        };

        match opcode {
            // RLC
            0x00..=0x07 => todo!("Not implemented"),
            // RRC
            0x08..=0x0F => todo!("Not implemented"),
            // RL
            0x10..=0x17 => todo!("Not implemented"),
            // RR
            0x18..=0x1F => todo!("Not implemented"),
            // SLA
            0x20..=0x27 => todo!("Not implemented"),
            // SRA
            0x28..=0x2F => todo!("Not implemented"),
            // SWAP
            0x30..=0x37 => todo!("Not implemented"),
            // SRL
            0x38..=0x3F => todo!("Not implemented"),
            // BIT
            0x40..=0x7F => {
                let bit: u8 = opcode - 0x40 / 8;
                self.bit_n_r(bit, target_value, is_hl)
            }
            // RES
            0x80..=0xBF => {
                let _bit: u8 = opcode - 0x40 / 8;
                todo!("Not implemented");
            }
            // SET
            _ => {
                let _bit: u8 = opcode - 0x40 / 8;
                todo!("Not implemented");
            }
        }
    }
}
