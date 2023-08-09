/// ADSR envelope.
#[cfg_attr(not(target_family = "wasm"), derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub struct Adsr {
    attack: u8,
    decay: u8,
    sustain: u8,
    release: u8,
}

impl Adsr {
    /// Build a new envelope.
    pub const fn new(attack: u8, decay: u8, sustain: u8, release: u8) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
        }
    }
}

impl Default for Adsr {
    fn default() -> Self {
        Self {
            attack: 0,
            decay: 0,
            sustain: 1,
            release: 0,
        }
    }
}

impl From<Adsr> for u32 {
    fn from(value: Adsr) -> Self {
        value.sustain as u32
            | ((value.release as u32) << 8)
            | ((value.decay as u32) << 16)
            | ((value.attack as u32) << 24)
    }
}

impl From<u32> for Adsr {
    fn from(value: u32) -> Self {
        let sustain = (value & 0xff) as u8;
        let release = ((value >> 8) & 0xff) as u8;
        let decay = ((value >> 16) & 0xff) as u8;
        let attack = ((value >> 24) & 0xff) as u8;

        Self {
            attack,
            decay,
            sustain,
            release,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adsr_to_u32() {
        assert_eq!(u32::from(Adsr::new(0, 0, 30, 0)), 0x1e);
        assert_eq!(u32::from(Adsr::new(30, 20, 30, 10)), 0x1e140a1e);
    }

    #[test]
    fn adsr_from_u32() {
        assert_eq!(
            Adsr::from(0x1e),
            Adsr {
                attack: 0,
                decay: 0,
                sustain: 30,
                release: 0
            }
        );
        assert_eq!(
            Adsr::from(0x1e140a1e),
            Adsr {
                attack: 30,
                decay: 20,
                sustain: 30,
                release: 10
            }
        );
    }
}
