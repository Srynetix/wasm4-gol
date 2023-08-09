/// Duty cycle.
#[repr(u8)]
#[cfg_attr(not(target_family = "wasm"), derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub enum DutyCycle {
    /// 12.5 %.
    C12_5 = 0,
    /// 25 %.
    C25 = 4,
    /// 50 %.
    C50 = 8,
    /// 75 %.
    C75 = 12,
}

impl From<u8> for DutyCycle {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::C12_5,
            4 => Self::C25,
            8 => Self::C50,
            12 => Self::C75,
            _ => panic!("unknown duty cycle: {value}"),
        }
    }
}

impl Default for DutyCycle {
    fn default() -> Self {
        Self::C12_5
    }
}
