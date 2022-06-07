use crate::{cpu::register::Reg16, memory::AddressSpace};

use super::{register::Reg8, Cpu};

enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

const TARGETS: [Target; 8] = [
    Target::B,
    Target::C,
    Target::D,
    Target::E,
    Target::H,
    Target::L,
    Target::HL,
    Target::A,
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

    // TODO Implemented other CB instructionis

    pub fn match_cb_prefix(&mut self, opcode: u8) -> Cycles {
        println!("CB: {:02X}", opcode);

        let mut is_hl = false;
        let target = &TARGETS[opcode as usize % 8];

        let target_value = match target {
            Target::B => self.get_r(Reg8::B).value(),
            Target::C => self.get_r(Reg8::C).value(),
            Target::D => self.get_r(Reg8::D).value(),
            Target::E => self.get_r(Reg8::E).value(),
            Target::H => self.get_r(Reg8::H).value(),
            Target::L => self.get_r(Reg8::L).value(),
            Target::HL => {
                is_hl = true;
                let address = self.get_r16(Reg16::HL);
                self.mmu.get(address)
            }
            Target::A => self.get_r(Reg8::A).value(),
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
