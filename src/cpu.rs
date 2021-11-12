// TODO Remove when not needed anymore
#[allow(dead_code)]
pub struct Cpu {
    // Registers
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    // a: u8,
    // b: u8,
    // c: u8,
    // d: u8,
    // e: u8,
    // f: u8,
    // h: u8,
    // l: u8,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn init(&mut self) {
        self.af = 0x01B0;
        self.bc = 0x0013;
        self.de = 0x00D8;
        self.hl = 0x014D;
        self.sp = 0xFFFE;
    }

    pub fn execute(&mut self) {
        let opcode: u8 = 0x08;

        match opcode {
            // NOP
            0x00 => {}
            _ => {}
        }
    }
}
