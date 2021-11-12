use memory::Memory;

// use crate::cpu::Cpu;

mod boot_rom;
mod cpu;
mod memory;

fn main() {
    let memory: Memory = Memory::new(String::from("./tetris.gb"));

    println!("{}", memory.get_rom_title());
    assert_eq!(0x7FFF, memory.rom.len() - 1);
}
