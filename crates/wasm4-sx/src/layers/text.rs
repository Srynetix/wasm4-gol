use wasm4_sys::{text, SCREEN_SIZE};

const CHARS_PER_LINE: usize = 20;
const CHAR_WIDTH: usize = SCREEN_SIZE as usize / CHARS_PER_LINE;
const CHAR_HEIGHT: usize = 8;

pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum TextVerticalAligment {
    Top,
    Middle,
    Bottom,
}

impl TextHorizontalAlignment {
    pub fn get_padding_x<T: AsRef<[u8]>>(&self, text: T) -> i32 {
        let len = text.as_ref().len() as i32;

        match *self {
            Self::Left => 0,
            Self::Center => ((CHARS_PER_LINE as i32 - len) / 2) * CHAR_WIDTH as i32,
            Self::Right => (CHARS_PER_LINE as i32 - len) * CHAR_WIDTH as i32,
        }
    }
}

impl TextVerticalAligment {
    pub fn get_padding_y(&self, line_count: usize, line_separation: i32) -> i32 {
        match *self {
            Self::Top => 0,
            Self::Middle => {
                ((SCREEN_SIZE as f32) / 2.0 - (CHAR_HEIGHT as f32 * line_count as f32) / 2.0) as i32
            }
            Self::Bottom => {
                SCREEN_SIZE as i32
                    - (CHAR_HEIGHT as i32 * line_count as i32)
                    - line_separation * (line_count - 1) as i32
            }
        }
    }
}

#[must_use]
pub struct Text<T: AsRef<[u8]>> {
    value: T,
    x: i32,
    y: i32,
    horizontal_alignment: Option<TextHorizontalAlignment>,
    vertical_alignment: Option<TextVerticalAligment>,
    padding_x: i32,
    padding_y: i32,
    line_separation: i32,
}

impl<T: AsRef<[u8]>> Text<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            horizontal_alignment: None,
            vertical_alignment: None,
            x: 0,
            y: 0,
            padding_x: 0,
            padding_y: 0,
            line_separation: 0,
        }
    }

    pub fn with_x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    pub fn with_padding_x(mut self, padding_x: i32) -> Self {
        self.padding_x = padding_x;
        self
    }

    pub fn with_padding_y(mut self, padding_y: i32) -> Self {
        self.padding_y = padding_y;
        self
    }

    pub fn with_horizontal_alignment(mut self, alignment: TextHorizontalAlignment) -> Self {
        self.horizontal_alignment = Some(alignment);
        self
    }

    pub fn with_vertical_alignment(mut self, alignment: TextVerticalAligment) -> Self {
        self.vertical_alignment = Some(alignment);
        self
    }

    pub fn with_line_separation(mut self, line_separation: i32) -> Self {
        self.line_separation = line_separation;
        self
    }

    pub fn draw(self) {
        let line_count = self.value.as_ref().split(|&v| v == b'\n').count();
        for (line_index, line) in self.value.as_ref().split(|&v| v == b'\n').enumerate() {
            self.draw_line(line_index, line_count, line);
        }
    }

    fn horizontal_padding<U: AsRef<[u8]>>(&self, text: U) -> i32 {
        match self.horizontal_alignment.as_ref() {
            Some(s) => s.get_padding_x(text) + self.padding_x,
            None => self.padding_x,
        }
    }

    fn vertical_padding(&self, line_count: usize) -> i32 {
        match self.vertical_alignment.as_ref() {
            Some(s) => s.get_padding_y(line_count, self.line_separation) + self.padding_y,
            None => self.padding_y,
        }
    }

    fn draw_line<U: AsRef<[u8]>>(&self, line_index: usize, line_count: usize, line: U) {
        let x = self.horizontal_padding(line.as_ref()) + self.x;
        let y = self.vertical_padding(line_count)
            + self.y
            + (CHAR_WIDTH as i32 + self.line_separation) * line_index as i32;

        text(line.as_ref(), x, y);
    }
}
