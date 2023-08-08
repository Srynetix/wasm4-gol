use std::{collections::HashMap, io::Write};

use byteorder::{LittleEndian, WriteBytesExt};
use wasm4_sx::*;

extern crate wasm4_stubs;

/// Track format
///
/// tone = {freq:u32}{dur:u32}{vol:u16}{flags:u16} = 12 B
/// note = {frame_num:u16}{voice_count:u8}{tones:&[tone]} = 3 + (T * 12) B
/// track = {notes:&[note]} = (3 + (T * 12)) * N B
///
/// T = 3
/// N = 60
/// => (3 + (3 * 12)) * 60 = 2340 B

struct Track {
    notes: Vec<Note>,
}

impl Track {
    pub fn new(frames: Vec<TrackKeyframe>) -> Self {
        Self { notes: Self::merge_frames(frames) }
    }

    fn merge_frames(frames: Vec<TrackKeyframe>) -> Vec<Note> {
        let mut map: HashMap<u16, Vec<Tone>> = HashMap::new();

        for frame in frames {
            for note in frame.notes {
                map.entry(note.key)
                    .and_modify(|v| v.extend(note.voices.clone()))
                    .or_insert_with(|| note.voices);
            }
        }

        let mut sorted_keys = map.keys().copied().collect::<Vec<_>>();
        sorted_keys.sort();
        sorted_keys
            .into_iter()
            .map(|k| Note::new(k, map.get(&k).unwrap().clone()))
            .collect()
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for note in &self.notes {
            note.write(writer)?;
        }

        if let Some(last_key) = self.get_last_key() {
            writer.write_u16::<LittleEndian>(last_key)?;
        }

        Ok(())
    }

    fn get_last_key(&self) -> Option<u16> {
        self.notes.iter().max_by_key(|x| x.key).map(|x| x.key)
    }
}

struct TrackKeyframe {
    notes: Vec<Note>,
}

impl TrackKeyframe {
    pub fn new(key: u16, patterns: Vec<Pattern>) -> Self {
        Self {
            notes: Self::merge_patterns(key, patterns),
        }
    }

    fn merge_patterns(key: u16, patterns: Vec<Pattern>) -> Vec<Note> {
        let mut map: HashMap<u16, Vec<Tone>> = HashMap::new();

        for pattern in patterns {
            for note in pattern.notes {
                map.entry(note.key + key)
                    .and_modify(|v| v.extend(note.voices.clone()))
                    .or_insert_with(|| note.voices);
            }
        }

        let mut sorted_keys = map.keys().copied().collect::<Vec<_>>();
        sorted_keys.sort();
        sorted_keys
            .into_iter()
            .map(|k| Note::new(k, map.get(&k).unwrap().clone()))
            .collect()
    }
}

#[derive(Clone)]
struct Pattern {
    notes: Vec<Note>,
}

impl Pattern {
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }
}

#[derive(Clone)]
struct Note {
    key: u16,
    voices: Vec<Tone>,
}

impl Note {
    pub fn new(key: u16, voices: Vec<Tone>) -> Self {
        Self { key, voices }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u16::<LittleEndian>(self.key)?;
        writer.write_u16::<LittleEndian>(self.voices.len() as u16)?;
        for tone in &self.voices {
            let (freq, dur, vol, flags) = tone.to_binary();
            writer.write_u32::<LittleEndian>(freq)?;
            writer.write_u32::<LittleEndian>(dur)?;
            writer.write_u16::<LittleEndian>(vol)?;
            writer.write_u16::<LittleEndian>(flags)?;
        }

        Ok(())
    }
}

const fn snare() -> Tone {
    Tone::builder()
        .with_frequency(FrequencySlide::new(220))
        .with_duration(Adsr::new(0, 4, 4, 0))
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

fn main() {
    let pat1 = Pattern::new(vec![
        Note::new(10, vec![bass()]),
        Note::new(30, vec![snare()]),
        Note::new(50, vec![bass()]),
        Note::new(60, vec![bass()]),
        Note::new(70, vec![snare()]),
    ]);

    let pat2 = Pattern::new(vec![
        Note::new(10, vec![bass()]),
        Note::new(20, vec![synth2()]),
        Note::new(30, vec![snare()]),
        Note::new(40, vec![synth2()]),
        Note::new(50, vec![bass()]),
        Note::new(60, vec![bass()]),
        Note::new(70, vec![snare()]),
        Note::new(80, vec![snare()]),
    ]);

    let pat3 = Pattern::new(vec![Note::new(0, vec![synth()])]);

    let track = Track::new(vec![TrackKeyframe::new(
        0,
        vec![pat1.clone()],
    ), TrackKeyframe::new(
        80,
        vec![pat2.clone()]
    ), TrackKeyframe::new(
        160,
        vec![pat1.clone()]
    ), TrackKeyframe::new(
        240,
        vec![pat3.clone()]
    )]);

    let mut vec = vec![];

    track.write(&mut vec).unwrap();
    println!("{:?}", vec);

    std::fs::write("song.bin", vec).unwrap();
}
