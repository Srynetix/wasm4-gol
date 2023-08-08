use super::{channel::Channel, duty_cycle::DutyCycle, panning::Panning};

/// Tone flags.
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
#[derive(Clone, Copy)]
pub struct ToneFlags {
    channel: Channel,
    duty_cycle: DutyCycle,
    panning: Panning,
}

impl ToneFlags {
    /// Create a tone flags builder.
    pub const fn builder() -> ToneFlagsBuilder {
        ToneFlagsBuilder::new()
    }

    /// Build new default tone flags.
    pub const fn new() -> Self {
        Self {
            channel: Channel::Pulse1,
            duty_cycle: DutyCycle::C12_5,
            panning: Panning::Center,
        }
    }
}

impl Default for ToneFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Tone flags builder.
pub struct ToneFlagsBuilder {
    flags: ToneFlags,
}

impl ToneFlagsBuilder {
    /// Create a tone flags builder.
    pub const fn new() -> Self {
        Self {
            flags: ToneFlags::new(),
        }
    }

    /// Set a channel.
    pub const fn with_channel(mut self, channel: Channel) -> Self {
        self.flags.channel = channel;
        self
    }

    /// Set a duty cycle value.
    pub const fn with_duty_cycle(mut self, duty_cycle: DutyCycle) -> Self {
        self.flags.duty_cycle = duty_cycle;
        self
    }

    /// Set panning.
    pub const fn with_panning(mut self, panning: Panning) -> Self {
        self.flags.panning = panning;
        self
    }

    /// Build the tone flags.
    pub const fn build(self) -> ToneFlags {
        self.flags
    }
}

impl Default for ToneFlagsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ToneFlags> for u16 {
    fn from(value: ToneFlags) -> Self {
        value.channel as u16 | ((value.duty_cycle as u16) << 2) | ((value.panning as u16) << 4)
    }
}

impl From<u16> for ToneFlags {
    fn from(value: u16) -> Self {
        let channel = Channel::from((value & 0b11) as u8);
        let duty_cycle = DutyCycle::from(((value >> 2) & 0b1100) as u8);
        let panning = Panning::from(((value >> 4) & 0b110000) as u8);

        Self {
            channel,
            duty_cycle,
            panning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_to_u16() {
        assert_eq!(
            u16::from(ToneFlags {
                channel: Channel::Pulse1,
                duty_cycle: DutyCycle::C12_5,
                panning: Panning::Center
            }),
            0
        );
        assert_eq!(
            u16::from(ToneFlags {
                channel: Channel::Pulse2,
                duty_cycle: DutyCycle::C75,
                panning: Panning::Right
            }),
            0x231
        );
    }

    #[test]
    fn flags_from_u16() {
        assert_eq!(
            ToneFlags::from(0),
            ToneFlags {
                channel: Channel::Pulse1,
                duty_cycle: DutyCycle::C12_5,
                panning: Panning::Center
            }
        );
        assert_eq!(
            ToneFlags::from(0x231),
            ToneFlags {
                channel: Channel::Pulse2,
                duty_cycle: DutyCycle::C75,
                panning: Panning::Right
            }
        );
    }
}
