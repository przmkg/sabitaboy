use crate::cpu::Cpu;

pub enum ExecutionResult {
    Continue,
    Stop,
}

pub fn execute(opcode: u8, cpu: &mut Cpu) -> ExecutionResult {
    println!("Opcode: {:#04X}", opcode);
    match opcode {
        // NOP
        0x00 => {}
        // JP a16
        0xC3 => jp_a16(cpu),
        0xAF => xor_a(cpu),
        0x21 => ld_d16(cpu, opcode),
        0x0E => ld_c_d8(cpu),
        0x06 => ld_b_d8(cpu),
        _ => {
            println!("Unimplemented: {:#04X}", opcode);
            return ExecutionResult::Stop;
        }
    }

    ExecutionResult::Continue
    // Check for interrupts
}

// JP a16, 3, 16
fn jp_a16(cpu: &mut Cpu) {
    let next = cpu.next_word();
    cpu.regs_mut().set_pc(next);
}

// XOR A, 1, 4
// Z
fn xor_a(cpu: &mut Cpu) {
    let a = cpu.regs().get_a();
    let result = a ^ a;

    if result == 0 {
        cpu.flags_mut().set_zero(true);
    }
}

// LD RR, d16, 3, 12
fn ld_d16(cpu: &mut Cpu, opcode: u8) {
    let value = cpu.next_word();

    match opcode {
        0x01 => cpu.regs_mut().set_bc(value),
        0x11 => cpu.regs_mut().set_de(value),
        0x21 => cpu.regs_mut().set_hl(value),
        0x31 => cpu.regs_mut().set_sp(value),
        _ => {}
    }
}

// TODO Refactor LD operations

// LD R, d8, 2, 8
fn ld_c_d8(cpu: &mut Cpu) {
    // TODO Does it load the immediate value ?
    let value = cpu.next_byte();
    cpu.regs_mut().set_c(value);
}

// LD R, d8, 2, 8
fn ld_b_d8(cpu: &mut Cpu) {
    // TODO Does it load the immediate value ?
    let value = cpu.next_byte();
    cpu.regs_mut().set_b(value);
}

