pub fn split_word(word: u16) -> (u8, u8) {
    let h = (word >> 8) as u8;
    let l = word as u8;

    (h, l)
}

pub fn get_word_from_bytes(h: u8, l: u8) -> u16 {
    (h as u16) << 8 & l as u16
}

// pub fn set_lower_byte(word: u16, value: u8) -> u16 {
//     (word & 0xF0) | value as u16
// }

// pub fn set_higher_byte(word: u16, value: u8) -> u16 {
//     (word & 0x0F) | ((value as u16) << 8)
// }

// #[cfg(test)]
// mod tests {
//     use crate::cpu::byteutils::{set_higher_byte, set_lower_byte};

//     #[test]
//     pub fn set_single_register() {
//         let a = 0b10110101_00101110;
//         let b = 0b11001001;

//         assert_eq!(0b10110101_11001001, set_lower_byte(a, b));
//         assert_eq!(0b11001001_00101110, set_higher_byte(a, b));
//     }
// }
