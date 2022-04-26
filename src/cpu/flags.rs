#[allow(dead_code)]
pub struct Flags {
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
    // interrupt: bool,
    // pending_interrupt_enabled: bool
}

#[allow(dead_code)]
impl Flags {
    pub fn new() -> Self {
        Self {
            zero: true,
            sub: false,
            half_carry: true,
            carry: true,
        }
    }

    pub fn zero(&self) -> bool {
        self.zero
    }

    pub fn sub(&self) -> bool {
        self.sub
    }

    pub fn half_carry(&self) -> bool {
        self.half_carry
    }

    pub fn carry(&self) -> bool {
        self.carry
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value
    }

    pub fn set_sub(&mut self, value: bool) {
        self.sub = value
    }

    pub fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value
    }
}
