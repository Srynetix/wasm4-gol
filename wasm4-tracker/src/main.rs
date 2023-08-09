extern crate wasm4_stubs;

use clap::Parser;
use std::{mem::size_of, path::Path};

use wasm4_sx::*;
use wasm4_tracker::*;

const fn snare() -> Tone {
    Tone::builder()
        .with_frequency(FrequencySlide::new(220))
        .with_duration(Adsr::new(0, 4, 4, 0))
        .with_volume(Volume::new(12))
        .with_flags(ToneFlags::builder().with_channel(Channel::Noise).build())
        .build()
}

const fn hat() -> Tone {
    Tone::builder()
        .with_frequency(FrequencySlide::new(330))
        .with_duration(Adsr::new(0, 2, 2, 0))
        .with_volume(Volume::new(12))
        .with_flags(ToneFlags::builder().with_channel(Channel::Noise).build())
        .build()
}

const fn synth2() -> Tone {
    Tone::builder()
        .with_frequency(FrequencySlide::new_slide(440, 880))
        .with_duration(Adsr::new(0, 0, 7, 0))
        .with_volume(Volume::new(25))
        .with_flags(ToneFlags::builder().with_channel(Channel::Pulse2).build())
        .build()
}

const fn synth() -> Tone {
    Tone::builder()
        .with_frequency(FrequencySlide::new_slide(220, 110))
        .with_duration(Adsr::new(0, 0, 7, 0))
        .with_volume(Volume::new(25))
        .with_flags(ToneFlags::builder().with_channel(Channel::Pulse1).build())
        .build()
}

const fn bass() -> Tone {
    Tone::builder()
        .with_frequency(FrequencySlide::new_slide(110, 55))
        .with_duration(Adsr::new(0, 0, 7, 0))
        .with_volume(Volume::new(25))
        .with_flags(ToneFlags::builder().with_channel(Channel::Triangle).build())
        .build()
}

fn build_track() -> Track {
    let pat1a = pattern!(
        rest!(N1_4),
        note!(N1_4, hat()),
        note!(N1_4, snare()),
        note!(N1_4, hat()),
        rest!(N1_4),
        note!(N1_4, hat()),
        note!(N1_4, snare()),
        note!(N1_8, hat())
    );

    let pat1b = pattern!(
        note!(N1_4, bass()),
        note!(N1_4, hat()),
        note!(N1_4, snare()),
        note!(N1_4, hat()),
        note!(N1_4, bass()),
        note!(N1_4, hat()),
        note!(N1_4, snare()),
        note!(N1_8, hat())
    );

    let pat2 = pattern!(
        rest!(N1),
        rest!(N1_2),
        note!(N1_4, synth()),
        note!(N1_8, synth2())
    );

    let pat3 = pattern!(
        rest!(N1),
        rest!(N1_2),
        note!(N1_4, synth2()),
        note!(N1_8, synth())
    );

    track!(
        frame!(pat1a),
        frame!(pat1a),
        frame!(pat1b),
        frame!(pat1b),
        frame!(pat1a, pat2),
        frame!(pat1a, pat3),
        frame!(pat1b, pat2),
        frame!(pat1b, pat3)
    )
}

fn render_track<P: AsRef<Path>>(track: Track, dest: P) {
    let mut vec = vec![];
    track.write(&mut vec).unwrap();
    track.print();

    println!("Length: {} bytes", vec.len() * size_of::<u8>());
    std::fs::write(dest.as_ref(), vec).unwrap();

    println!("Written to {}", dest.as_ref().to_string_lossy());
}

fn main() {
    let args = Args::parse();
    render_track(build_track(), args.output);
}
