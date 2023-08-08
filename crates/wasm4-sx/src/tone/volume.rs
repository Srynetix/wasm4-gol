/// Tone volume.
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub struct Volume {
    sustain: u8,
    peak: u8,
}

impl Volume {
    /// Build a new volume with sustain.
    pub const fn new(value: u8) -> Self {
        Self {
            sustain: value,
            peak: 0,
        }
    }

    /// Build a new volume with sustain and peak.
    pub const fn new_with_peak(sustain: u8, peak: u8) -> Self {
        Self { sustain, peak }
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            sustain: 100,
            peak: 0,
        }
    }
}

impl From<Volume> for u16 {
    fn from(value: Volume) -> Self {
        value.sustain as u16 | (value.peak as u16) << 8
    }
}

impl From<u16> for Volume {
    fn from(value: u16) -> Self {
        let sustain = (value & 0xff) as u8;
        let peak = ((value >> 8) & 0xff) as u8;

        Self { sustain, peak }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_to_u16() {
        assert_eq!(u16::from(Volume::new(50)), 0x32);
        assert_eq!(u16::from(Volume::new_with_peak(50, 75)), 0x4b32);
    }

    #[test]
    fn volume_from_u16() {
        assert_eq!(
            Volume::from(0x32),
            Volume {
                sustain: 50,
                peak: 0
            }
        );
        assert_eq!(
            Volume::from(0x4b32),
            Volume {
                sustain: 50,
                peak: 75
            }
        );
    }
}
