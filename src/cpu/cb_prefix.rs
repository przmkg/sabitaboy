use crate::{cpu::register::Reg16, memory::AddressSpace};

use super::{instr::Target, register::Reg8, Cpu};

const TARGETS: [Target; 8] = [
    Target::Register8(Reg8::B),
    Target::Register8(Reg8::C),
    Target::Register8(Reg8::D),
    Target::Register8(Reg8::E),
    Target::Register8(Reg8::H),
    Target::Register8(Reg8::L),
    Target::RegisterAddress16(Reg16::HL),
    Target::Register8(Reg8::A),
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

        let target_value = match &TARGETS[opcode as usize % 8] {
            Target::Register8(r8) => self.get_r(*r8).value(),
            Target::RegisterAddress16(r16) => {
                is_hl = true;
                let address = self.get_r16(*r16);
                self.mmu.get(address)
            }
            _ => panic!("Wrong target for a CB prefix"),
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
