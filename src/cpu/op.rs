use crate::{cpu::Cpu, memory::AddressSpace};

use super::{
    flags::Flags,
    register::{Reg, Register, Registers},
};

pub fn execute(opcode: u8, cpu: &mut Cpu) -> u8 {
    println!("Opcode: {:#04X}", opcode);

    let regs = cpu.regs().clone();

    match opcode {
        // NOP
        0x00 => 4,
        0xC3 => jp_a16(cpu),
        0xAF => xor_a(cpu),
        // LD RR, d16
        0x01 => {
            let value = cpu.next_word();
            ld_rr_d16(&mut cpu.regs_mut(), Reg::BC, value)
        }
        0x11 => {
            let value = cpu.next_word();
            ld_rr_d16(&mut cpu.regs_mut(), Reg::DE, value)
        }
        0x21 => {
            let value = cpu.next_word();
            ld_rr_d16(&mut cpu.regs_mut(), Reg::HL, value)
        }
        0x31 => {
            let value = cpu.next_word();
            ld_rr_d16(&mut cpu.regs_mut(), Reg::SP, value)
        }
        0x05 => {
            let reg_a = &mut cpu.regs_mut().a;
            // TODO Implement that
            dec_r(reg_a)
        }
        // LD R, d8
        0x06 => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().b, value)
        }
        0x16 => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().d, value)
        }
        0x26 => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().h, value)
        }
        0x0E => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().c, value)
        }
        0x1E => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().e, value)
        }
        0x2E => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().l, value)
        }
        0x3E => {
            let value = cpu.next_byte();
            ld_r_d8(&mut cpu.regs_mut().a, value)
        }
        // LD (RR), R
        0x02 => {
            let (target_address, value) = (regs.get_bc(), regs.a.value());
            ld_a16_r(cpu, target_address, value)
        }
        0x12 => {
            let (target_address, value) = (regs.get_de(), regs.a.value());
            ld_a16_r(cpu, target_address, value)
        }
        0x22 => {
            let value = regs.a.value();
            ld_hl_r(cpu, value, HLAction::Inc)
        }
        0x32 => {
            let value = regs.a.value();
            ld_hl_r(cpu, value, HLAction::Dec)
        }
        _ => {
            panic!("Unimplemented: {:#04X}", opcode);
        }
    }

    // Check for interrupts
}

type Cycles = u8;

// JP a16, 3, 16
fn jp_a16(cpu: &mut Cpu) -> Cycles {
    let next = cpu.next_word();
    cpu.regs_mut().set_pc(next);

    16
}

// XOR A, 1, 4
// Z
fn xor_a(cpu: &mut Cpu) -> Cycles {
    let a = cpu.regs().a.value();
    let result = a ^ a;

    if result == 0 {
        cpu.flags_mut().set_zero(true);
    }

    4
}

// LD RR, d16, 3, 12
fn ld_rr_d16(regs: &mut Registers, target_register: Reg, value: u16) -> Cycles {
    match target_register {
        Reg::BC => regs.set_bc(value),
        Reg::DE => regs.set_de(value),
        Reg::HL => regs.set_hl(value),
        Reg::SP => regs.set_sp(value),
        _ => panic!("LD RR, d16, Unknown register: {:?}", target_register),
    }

    12
}

// LD R, d8, 2, 8
fn ld_r_d8(target_register: &mut Register<u8>, value: u8) -> Cycles {
    target_register.set(value);

    8
}

pub enum HLAction {
    Inc,
    Dec,
    None,
}

// LD (HL), R, 1, 8
fn ld_hl_r(cpu: &mut Cpu, value: u8, action: HLAction) -> Cycles {
    let hl_value = cpu.regs().get_hl();
    cpu.mmu_mut().set(hl_value, value);

    match action {
        HLAction::Inc => cpu.regs_mut().set_hl(hl_value.saturating_add(1)),
        HLAction::Dec => cpu.regs_mut().set_hl(hl_value.saturating_sub(1)),
        HLAction::None => {}
    }

    8
}

// LD (RR), R, 1, 8
fn ld_a16_r(cpu: &mut Cpu, target_address: u16, value: u8) -> Cycles {
    cpu.mmu_mut().set(target_address, value);

    8
}

// DEC R, 1, 4
// Z 1 H
fn dec_r(target_register: &mut Register<u8>) -> Cycles {
    let value = target_register.value();

    target_register.dec();

    4
}
