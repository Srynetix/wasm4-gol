/// Frequency slide.
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub struct FrequencySlide {
    first: u16,
    second: u16,
}

impl Default for FrequencySlide {
    fn default() -> Self {
        Self {
            first: 220,
            second: 0,
        }
    }
}

impl FrequencySlide {
    /// Build a new frequency.
    pub const fn new(value: u16) -> Self {
        Self {
            first: value,
            second: 0,
        }
    }

    /// Build a new frequency slide.
    pub const fn new_slide(from: u16, to: u16) -> Self {
        Self {
            first: from,
            second: to,
        }
    }
}

impl From<FrequencySlide> for u32 {
    fn from(value: FrequencySlide) -> Self {
        value.first as u32 | (value.second as u32) << 16
    }
}

impl From<u32> for FrequencySlide {
    fn from(value: u32) -> Self {
        let first = (value & 0xffff) as u16;
        let second = ((value >> 16) & 0xffff) as u16;

        Self { first, second }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_slide_to_u32() {
        assert_eq!(u32::from(FrequencySlide::new(440)), 0x1b8);
        assert_eq!(u32::from(FrequencySlide::new_slide(440, 880)), 0x37001b8);
    }

    #[test]
    fn frequency_slide_from_u32() {
        assert_eq!(
            FrequencySlide::from(0x1b8),
            FrequencySlide {
                first: 440,
                second: 0
            }
        );
        assert_eq!(
            FrequencySlide::from(0x37001b8),
            FrequencySlide {
                first: 440,
                second: 880
            }
        );
    }
}
