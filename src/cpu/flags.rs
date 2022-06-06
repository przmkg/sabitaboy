pub struct Flags {
    pub zero: bool,
    pub sub: bool,
    pub half_carry: bool,
    pub carry: bool,
    // interrupt: bool,
    // pending_interrupt_enabled: bool
}

impl Flags {
    pub fn new() -> Self {
        Self {
            zero: true,
            sub: false,
            half_carry: true,
            carry: true,
        }
    }
}
