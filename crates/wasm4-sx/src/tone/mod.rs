mod adsr;
mod channel;
mod duty_cycle;
mod frequency_slide;
mod panning;

#[allow(clippy::module_inception)]
mod tone;

mod tone_flags;
mod volume;

pub use adsr::Adsr;
pub use channel::Channel;
pub use duty_cycle::DutyCycle;
pub use frequency_slide::FrequencySlide;
pub use panning::Panning;
pub use tone::{Tone, ToneBuilder};
pub use tone_flags::{ToneFlags, ToneFlagsBuilder};
pub use volume::Volume;
