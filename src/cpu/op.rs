use crate::{cpu::Cpu, memory::AddressSpace};

use super::register::Reg;

pub fn execute(opcode: u8, cpu: &mut Cpu) -> u8 {
    println!("Opcode: {:#04X}", opcode);

    let regs = cpu.regs().clone();

    match opcode {
        // NOP
        0x00 => 4,
        0xC3 => jp_a16(cpu),
        0xAF => xor_a(cpu),
        0x21 => ld_d16(cpu, opcode),
        // LD R, d8
        0x06 => ld_r_d8(cpu, Reg::B),
        0x16 => ld_r_d8(cpu, Reg::D),
        0x26 => ld_r_d8(cpu, Reg::H),
        0x0E => ld_r_d8(cpu, Reg::C),
        0x1E => ld_r_d8(cpu, Reg::E),
        0x2E => ld_r_d8(cpu, Reg::L),
        0x3E => ld_r_d8(cpu, Reg::A),
        // LD (RR), R
        0x02 => ld_a16_r(cpu, regs.bc.value().clone(), regs.get_a(), HLAction::None),
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
    let a = cpu.regs().get_a();
    let result = a ^ a;

    if result == 0 {
        cpu.flags_mut().set_zero(true);
    }

    4
}

// LD RR, d16, 3, 12
fn ld_d16(cpu: &mut Cpu, opcode: u8) -> Cycles {
    let value = cpu.next_word();

    match opcode {
        0x01 => cpu.regs_mut().set_bc(value),
        0x11 => cpu.regs_mut().set_de(value),
        0x21 => cpu.regs_mut().set_hl(value),
        0x31 => cpu.regs_mut().set_sp(value),
        _ => {}
    }

    12
}

// LD R, d8, 2, 8
fn ld_r_d8(cpu: &mut Cpu, target_register: Reg) -> Cycles {
    let value = cpu.next_byte();

    match target_register {
        Reg::A => cpu.regs_mut().set_a(value),
        Reg::B => cpu.regs_mut().set_b(value),
        Reg::C => cpu.regs_mut().set_c(value),
        Reg::D => cpu.regs_mut().set_d(value),
        Reg::E => cpu.regs_mut().set_e(value),
        Reg::H => cpu.regs_mut().set_h(value),
        Reg::L => cpu.regs_mut().set_l(value),
        _ => panic!("LD R, d8: Unknown register {:?}", target_register),
    }

    8
}

pub enum HLAction {
    Inc,
    Dec,
    None,
}

// LD (RR), R, 1, 8
fn ld_a16_r(cpu: &mut Cpu, target_address: u16, value: u8, action: HLAction) -> Cycles {
    cpu.mmu_mut().set(target_address, value);

    match action {
        HLAction::Inc => cpu.regs_mut().hl.inc(),
        HLAction::Dec => cpu.regs_mut().hl.dec(),
        HLAction::None => {}
    }

    8
}
