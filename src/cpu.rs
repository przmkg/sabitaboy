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

    flags: Flags,
}
pub struct Flags {
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
    // interrupt: bool,
    // pending_interrupt_enabled: bool
}

impl Cpu {
    pub fn power_up(&mut self) {

    }

    pub fn new() -> Self {
        Self {
            af: 0x01B0,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,
            sp: 0xFFFE,
            pc: 0x0100,

            flags: Flags {
                zero: true,
                sub: false,
                half_carry: true,
                carry: true,
            },
        }
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
