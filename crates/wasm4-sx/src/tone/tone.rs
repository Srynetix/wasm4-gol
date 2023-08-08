use wasm4_sys::tone;

use super::{adsr::Adsr, frequency_slide::FrequencySlide, tone_flags::ToneFlags, volume::Volume};

/// Tone.
#[derive(Clone, Default)]
pub struct Tone {
    /// Frequency.
    pub frequency: FrequencySlide,
    /// Duration.
    pub duration: Adsr,
    /// Volume.
    pub volume: Volume,
    /// Flags.
    pub flags: ToneFlags,
}

impl Tone {
    /// Create a new tone builder.
    pub const fn builder() -> ToneBuilder {
        ToneBuilder::new()
    }

    /// Build a new default tone.
    pub const fn new() -> Self {
        Self {
            frequency: FrequencySlide::new(220),
            duration: Adsr::new(0, 0, 1, 0),
            volume: Volume::new(100),
            flags: ToneFlags::new(),
        }
    }

    /// Convert tone to binary
    pub fn to_binary(&self) -> (u32, u32, u16, u16) {
        (
            u32::from(self.frequency),
            u32::from(self.duration),
            u16::from(self.volume),
            u16::from(self.flags),
        )
    }

    /// Play the tone.
    pub fn play(&self) {
        tone(
            self.frequency.into(),
            self.duration.into(),
            u16::from(self.volume) as u32,
            u16::from(self.flags) as u32,
        );
    }
}

/// Tone builder.
pub struct ToneBuilder {
    tone: Tone,
}

impl ToneBuilder {
    /// Create a new tone builder.
    pub const fn new() -> Self {
        Self { tone: Tone::new() }
    }

    /// Set the tone frequency.
    pub const fn with_frequency(mut self, frequency: FrequencySlide) -> Self {
        self.tone.frequency = frequency;
        self
    }

    /// Set the tone duration.
    pub const fn with_duration(mut self, duration: Adsr) -> Self {
        self.tone.duration = duration;
        self
    }

    /// Set the tone volume.
    pub const fn with_volume(mut self, volume: Volume) -> Self {
        self.tone.volume = volume;
        self
    }

    /// Set the tone flags.
    pub const fn with_flags(mut self, flags: ToneFlags) -> Self {
        self.tone.flags = flags;
        self
    }

    /// Build the tone.
    pub const fn build(self) -> Tone {
        self.tone
    }

    /// Play the tone.
    pub fn play(&self) {
        self.tone.play()
    }
}

impl Default for ToneBuilder {
    fn default() -> Self {
        Self::new()
    }
}
