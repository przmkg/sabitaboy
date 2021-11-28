use crate::cpu::Cpu;

pub fn execute(opcode: u8, cpu: &mut Cpu) {
    println!("Opcode: {:#04X}", opcode);
    match opcode {
        // NOP
        0x00 => {}
        // JP a16
        0xC3 => jp_a16(cpu),
        0xAF => xor_a(cpu),
        _ => {}
    }

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
