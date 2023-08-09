use std::{collections::HashMap, io::Write};

use byteorder::{LittleEndian, WriteBytesExt};
use wasm4_sx::{Engine, Tone};

/// Track format
///
/// tone = {freq:u32}{dur:u32}{vol:u16}{flags:u16} = 12 B
/// note = {frame_num:u16}{voice_count:u8}{tones:&[tone]} = 3 + (T * 12) B
/// track = {notes:&[note]} = (3 + (T * 12)) * N B
///
/// T = 3
/// N = 60
/// => (3 + (3 * 12)) * 60 = 2340 B

#[derive(Debug, Clone, Copy)]
pub enum NoteValue {
    N8,    // 8
    N4,    // 4
    N2,    // 2
    N1,    // 1,
    N1_2,  // 1/2
    N1_4,  // 1/4
    N1_8,  // 1/8
    N1_16, // 1/16
    N1_32, // 1/32
    N1_64, // 1/64
}

impl NoteValue {
    pub fn as_frame_count(&self) -> u16 {
        match &self {
            Self::N8 => Engine::FPS as u16 * 8,
            Self::N4 => Engine::FPS as u16 * 4,
            Self::N2 => Engine::FPS as u16 * 2,
            Self::N1 => Engine::FPS as u16,
            Self::N1_2 => Engine::FPS as u16 / 2,
            Self::N1_4 => Engine::FPS as u16 / 4,
            Self::N1_8 => Engine::FPS as u16 / 8,
            Self::N1_16 => Engine::FPS as u16 / 16,
            Self::N1_32 => Engine::FPS as u16 / 32,
            Self::N1_64 => Engine::FPS as u16 / 64,
        }
    }
}

#[derive(Debug)]
pub struct Track {
    notes: Vec<Note>,
}

impl Track {
    pub fn new(frames: Vec<TrackKeyframe>) -> Self {
        Self {
            notes: Self::merge_frames(frames),
        }
    }

    fn merge_frames(frames: Vec<TrackKeyframe>) -> Vec<Note> {
        let mut notes = vec![];
        let mut last_key = 0;

        for frame in frames {
            let mut last_frame_key = 0;

            for mut note in frame.notes {
                last_frame_key = note.key;
                note.key += last_key;
                notes.push(note);
            }

            last_key += last_frame_key;
        }

        notes
    }

    pub fn print(&self) {
        println!("TRACK");
        print_notes(&self.notes);
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

#[derive(Debug, Clone)]
pub struct TrackKeyframe {
    notes: Vec<Note>,
}

impl TrackKeyframe {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self {
            notes: Self::merge_patterns(patterns),
        }
    }

    pub fn print(&self) {
        println!("KEYFRAME");
        print_notes(&self.notes);
    }

    fn merge_patterns(patterns: Vec<Pattern>) -> Vec<Note> {
        let mut map: HashMap<u16, Vec<Tone>> = HashMap::new();
        for pattern in patterns {
            for note in pattern.notes {
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
}

#[derive(Debug, Clone)]
pub struct Pattern {
    notes: Vec<Note>,
}

impl Pattern {
    pub fn new(note_defs: Vec<NoteDef>) -> Self {
        let mut notes = vec![];
        let mut note_cursor = 0;
        let mut last_note_duration = 0;

        for def in note_defs {
            let note_duration = def.duration.as_frame_count();
            last_note_duration = note_duration;

            notes.push(Note::new(note_cursor, def.voices));
            note_cursor += note_duration;
        }

        if last_note_duration > 0 {
            notes.push(Note::new(note_cursor + last_note_duration, vec![]));
        }

        Self { notes }
    }

    pub fn print(&self) {
        println!("PATTERN");
        print_notes(&self.notes);
    }
}

fn print_notes(notes: &[Note]) {
    for (idx, note) in notes.iter().enumerate() {
        println!(
            "[{}] Note key={} voices_count={}",
            idx,
            note.key,
            note.voices.len()
        );
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub key: u16,
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

pub struct NoteDef {
    duration: NoteValue,
    voices: Vec<Tone>,
}

impl NoteDef {
    pub fn new(duration: NoteValue, voices: Vec<Tone>) -> Self {
        Self { duration, voices }
    }
}
