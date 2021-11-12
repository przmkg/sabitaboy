use super::address_space::AddressSpace;

pub struct Rom {
    data: Vec<u8>,
}

impl AddressSpace for Rom {
    fn get(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) -> () {
        self.data[address as usize] = value;
    }
}

impl Rom {
    fn new(&mut self, data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }
}
