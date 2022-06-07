use memory::Cartridge;

use crate::{cpu::Cpu, memory::Mmu};

mod boot_rom;
mod cpu;
mod memory;

fn main() {
    let cartridge = Cartridge::new(String::from("./tetris.gb"));
    let mut mmu = Mmu::new(cartridge);
    mmu.power_up();

    println!("Booting game: {}", mmu.cartridge().get_rom_title());

    let mut cpu = Cpu::new(&mut mmu);

    cpu.power_up();

    loop {
        // TODO Redo the whole loop
        cpu.execute();
    }
}
