pub trait AddressSpace {
    fn get(&self, address: u16) -> u8;

    fn set(&mut self, address: u16, value: u8);
}