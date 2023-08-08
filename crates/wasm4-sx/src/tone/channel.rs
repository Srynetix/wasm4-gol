/// Tone channel (waveform type).
#[repr(u8)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub enum Channel {
    /// First type of square wave.
    Pulse1 = 0,
    /// Second type of square wave.
    Pulse2 = 1,
    /// Triangle wave.
    Triangle = 2,
    /// Noise.
    Noise = 3,
}

impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Pulse1,
            1 => Self::Pulse2,
            2 => Self::Triangle,
            3 => Self::Noise,
            _ => panic!("unknown channel: {value}"),
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::Pulse1
    }
}
