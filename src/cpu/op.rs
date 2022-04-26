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
        0x21 => ld_a16(cpu, opcode),
        _ => return ExecutionResult::Stop,
    }

    ExecutionResult::Continue
    // Check for interrupts
}

// JP a16, 3, 16
fn jp_a16(cpu: &mut Cpu) {
    let next = cpu.next_word();
    println!("JP ${:#06X}", next);
    cpu.set_pc(next);
}

// XOR A, 1, 4
// Z
fn xor_a(cpu: &mut Cpu) {
    let a = cpu.get_a();
    let result = a ^ a;

    if result == 0 {
        cpu.set_fz(true);
    }
}

// LD R, a16, 3, 12
fn ld_a16(cpu: &mut Cpu, opcode: u8) {
    let value = cpu.next_word();

    println!("LD hl, {:#06X}", value);

    match opcode {
        0x01 => cpu.set_bc(value),
        0x11 => cpu.set_de(value),
        0x21 => cpu.set_hl(value),
        0x31 => cpu.set_sp(value),
        _ => {}
    }

    cpu.print_registers();
}
