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

    fn get_word(&self, address: u16) -> u16 {
        let h = (self.data[(address + 1) as usize] as u16) << 8;
        let l = self.data[address as usize] as u16;

        h | l
    }

    fn set_word(&mut self, address: u16, value: u16) {
        let val = value as u8;
        let next_val = (value >> 8) as u8;

        self.data[address as usize] = val;
        self.data[(address + 1) as usize] = next_val;
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
