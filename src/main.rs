use memory::Cartridge;

use crate::memory::Mmu;


// use crate::cpu::Cpu;

mod memory;
mod boot_rom;
mod cpu;

fn main() {
    let cartridge = Cartridge::new(String::from("./tetris.gb"));
    let mut mmu = Mmu { cartridge };
    mmu.power_up();

    // println!("{}", memory.get_rom_title());
    // assert_eq!(0x7FFF, memory.rom.len() - 1);
}
