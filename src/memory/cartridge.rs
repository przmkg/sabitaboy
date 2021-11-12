use std::{fs::File, io::Read};

use super::address_space::AddressSpace;

pub struct Cartridge {
    data: Vec<u8>,
}

impl AddressSpace for Cartridge {
    fn get(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

impl Cartridge {
    pub fn new(rom_path: String) -> Self {
        let mut rom_file = File::open(rom_path).unwrap();
        let mut data: Vec<u8> = Vec::new();
        rom_file.read_to_end(&mut data).unwrap();

        Self { data }
    }

    pub fn get_rom_title(&self) -> &str {
        std::str::from_utf8(&self.data[0x134..0x143])
            .unwrap()
            .trim_end()
    }
}
