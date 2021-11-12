use std::fs::File;
use std::io::{Read, Result};

pub struct Memory {
    pub rom: Vec<u8>,
}

impl Memory {
    pub fn new(rom_path: String) -> Self {
        Self {
            rom: Self::load_rom(rom_path).unwrap(),
        }
    }

    fn load_rom(rom_path: String) -> Result<Vec<u8>> {
        let mut rom_file = File::open(rom_path)?;
        let mut rom_data: Vec<u8> = Vec::new();
        rom_file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }

    pub fn get_rom_title(&self) -> &str {
        std::str::from_utf8(&self.rom[0x134..0x143])
            .unwrap()
            .trim_end()
    }
}
