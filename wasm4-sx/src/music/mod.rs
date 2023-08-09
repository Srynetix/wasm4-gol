use byteorder::{ByteOrder, LittleEndian};

use crate::ToneBuilder;

/// Track reader.
pub struct TrackReader<'a> {
    stream: &'a [u8],
    cursor: u16,
}

impl<'a> TrackReader<'a> {
    /// Build a new track reader.
    pub const fn new(stream: &'a [u8]) -> Self {
        Self { stream, cursor: 0 }
    }

    /// Play the track using a tick count.
    pub fn play_tick(&mut self, tick_count: usize) {
        let cursor = self.cursor;
        let stream = &self.stream[cursor as usize..];
        let reader = Reader::new(stream);

        if reader.eof() {
            return;
        }

        let last_value = LittleEndian::read_u16(&self.stream[self.stream.len() - 2..]);

        let mut bytes_read = 0;
        let (next_key, reader) = reader.read_u16();
        bytes_read += 2;

        if reader.eof() {
            // Next key was the last key.
            // Time to loop!
            self.cursor = 0;
            return;
        }

        if next_key as usize == tick_count % (last_value + 1) as usize {
            bytes_read += self.play_notes(reader);
            self.cursor = cursor + bytes_read as u16;

            // Check for next sample at same key.
            self.play_tick(tick_count)
        }
    }

    fn play_notes(&self, reader: Reader<'a>) -> usize {
        let mut bytes_read = 0;
        let (voice_count, reader) = reader.read_u16();
        bytes_read += 2;

        let mut loop_reader = reader;

        for _ in 0..voice_count {
            let (freq, reader) = loop_reader.read_u32();
            let (dur, reader) = reader.read_u32();
            let (vol, reader) = reader.read_u16();
            let (flags, reader) = reader.read_u16();
            bytes_read += 12;

            let tone = ToneBuilder::default()
                .with_frequency(freq.into())
                .with_duration(dur.into())
                .with_volume(vol.into())
                .with_flags(flags.into())
                .build();

            tone.play();

            loop_reader = reader;
        }

        bytes_read
    }
}

struct Reader<'a> {
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn eof(&self) -> bool {
        self.data.is_empty()
    }

    pub fn read_u16(&self) -> (u16, Reader<'a>) {
        let value = LittleEndian::read_u16(self.data);
        (value, Self::new(&self.data[2..]))
    }

    pub fn read_u32(&self) -> (u32, Reader<'a>) {
        let value = LittleEndian::read_u32(self.data);
        (value, Self::new(&self.data[4..]))
    }
}
