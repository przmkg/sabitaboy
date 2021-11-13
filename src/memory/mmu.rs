use super::address_space::AddressSpace;
use super::cartridge::Cartridge;
use super::{bytes_to_word, word_to_bytes};

pub struct Mmu {
    pub cartridge: Cartridge,
}

impl Mmu {
    pub fn power_up(&mut self) {
        self.init_memory();
    }

    fn init_memory(&mut self) {
        self.set(0xFF05, 0x00);
        self.set(0xFF06, 0x00);
        self.set(0xFF07, 0x00);
        self.set(0xFF10, 0x80);
        self.set(0xFF11, 0xBF);
        self.set(0xFF12, 0xF3);
        self.set(0xFF14, 0xBF);
        self.set(0xFF16, 0x3F);
        self.set(0xFF17, 0x00);
        self.set(0xFF19, 0xBF);
        self.set(0xFF1A, 0x7F);
        self.set(0xFF1B, 0xFF);
        self.set(0xFF1C, 0x9F);
        self.set(0xFF1E, 0xBF);
        self.set(0xFF20, 0xFF);
        self.set(0xFF21, 0x00);
        self.set(0xFF22, 0x00);
        self.set(0xFF23, 0xBF);
        self.set(0xFF24, 0x77);
        self.set(0xFF25, 0xF3);
        self.set(0xFF26, 0xF1); // SGB is 0xF0
        self.set(0xFF40, 0x91);
        self.set(0xFF42, 0x00);
        self.set(0xFF43, 0x00);
        self.set(0xFF45, 0x00);
        self.set(0xFF47, 0xFC);
        self.set(0xFF48, 0xFF);
        self.set(0xFF49, 0xFF);
        self.set(0xFF4A, 0x00);
        self.set(0xFF4B, 0x00);
        self.set(0xFFFF, 0x00);
    }
}

impl AddressSpace for Mmu {
    fn get(&self, address: u16) -> u8 {
        match address {
            // ROM Bank
            // Fixed until 0x3FFF
            0x0000..=0x7FFF => self.cartridge.get(address),

            // Video RAM
            0x8000..=0x9FFF => 0x00,

            // Switchable RAM Bank
            0xA000..=0xBFFF => 0x00,

            // Internal RAM (WRAM)
            0xC000..=0xDFFF => 0x00,

            // Echo of 8kB Internal RAM
            0xE000..=0xFDFF => 0x00,

            // Sprite Attrib Memory (OAM)
            0xFE00..=0xFE9F => 0x00,

            // Empty but unusable for I/O
            0xFEA0..=0xFEFF => 0x00,

            // I/O Registers
            // Unusable from 0xFF4C
            0xFF00..=0xFF7F => 0x00,

            // High RAM
            0xFF80..=0xFFFE => 0x00,

            // Interrupt Enable Register
            0xFFFF => 0x00,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            // ROM Bank
            // Fixed until 0x3FFF
            0x0000..=0x7FFF => self.cartridge.set(address, value),

            // Video RAM
            0x8000..=0x9FFF => {}

            // Switchable RAM Bank
            0xA000..=0xBFFF => {}

            // Internal RAM (WRAM)
            0xC000..=0xDFFF => {}

            // Echo of 8kB Internal RAM
            0xE000..=0xFDFF => {}

            // Sprite Attrib Memory (OAM)
            0xFE00..=0xFE9F => {}

            // Empty but unusable for I/O
            0xFEA0..=0xFEFF => {}

            // I/O Registers
            // Unusable from 0xFF4C
            0xFF00..=0xFF7F => {}

            // High RAM
            0xFF80..=0xFFFE => {}

            // Interrupt Enable Register
            0xFFFF => {}
        }
    }

    fn get_word(&self, address: u16) -> u16 {
        bytes_to_word(self.get(address + 1), self.get(address))
    }

    // TODO Not sure if any of this works
    fn set_word(&mut self, address: u16, value: u16) {
        let (h, l) = word_to_bytes(value);
        self.set(address + 1, h);
        self.set(address, l);
    }
}
