pub trait AddressSpace {
    fn get(&self, address: u16) -> u8;

    fn set(&mut self, address: u16, value: u8);

    fn get_word(&self, address: u16) -> u16;

    fn set_word(&mut self, address: u16, value: u16);
}
