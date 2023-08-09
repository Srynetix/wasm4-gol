/// Panning.
#[repr(u8)]
#[cfg_attr(not(target_family = "wasm"), derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub enum Panning {
    /// Pan center.
    Center = 0,
    /// Pan left.
    Left = 16,
    /// Pan right.
    Right = 32,
}

impl Default for Panning {
    fn default() -> Self {
        Self::Center
    }
}

impl From<u8> for Panning {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Center,
            16 => Self::Left,
            32 => Self::Right,
            _ => panic!("unknown panning value: {value}"),
        }
    }
}
