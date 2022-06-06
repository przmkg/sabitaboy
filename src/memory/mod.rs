mod address_space;
mod cartridge;
mod mmu;

pub use address_space::AddressSpace;
pub use cartridge::Cartridge;
pub use mmu::Mmu;

/// Converts two bytes to a single word
pub fn bytes_to_word(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

/// Converts a word into two bytes, first tuple value being the high value
pub fn word_to_bytes(w: u16) -> (u8, u8) {
    let h = (w >> 8) as u8;
    let l = w as u8;
    (h, l)
}

// TODO Move tests somewhere else
mod tests {
    

    #[test]
    pub fn test_bytes_to_word() {
        assert_eq!(0xC34F, bytes_to_word(0xC3, 0x4F));
    }

    #[test]
    pub fn test_word_to_bytes() {
        assert_eq!((0xC3, 0x4F), word_to_bytes(0xC34F));
    }
}
