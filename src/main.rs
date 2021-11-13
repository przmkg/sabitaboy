use memory::Cartridge;

use crate::{cpu::Cpu, memory::Mmu};

mod boot_rom;
mod cpu;
mod memory;

fn main() {
    let cartridge = Cartridge::new(String::from("./tetris.gb"));
    let mut mmu = Mmu { cartridge };
    mmu.power_up();

    let mut cpu = Cpu::new(&mut mmu);

    cpu.execute();
    cpu.execute();

    // assert_eq!(mmu.get(0x0100), 0x00);
    // assert_eq!(mmu.get(cpu.get_pc()), 0xC3);
    assert_eq!(cpu.get_pc(), 0x0150);

    println!("{}", mmu.cartridge.get_rom_title());
}
