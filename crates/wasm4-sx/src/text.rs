use wasm4_sys::{text, SCREEN_SIZE};

const CHARS_PER_LINE: usize = 20;
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 8;

/// Horizontal alignment.
#[derive(Clone, Copy)]
pub enum TextHorizontalAlignment {
    /// Align left.
    Left,
    /// Align center.
    Center,
    /// Align right.
    Right,
}

/// Vertical alignment.
#[derive(Clone, Copy)]
pub enum TextVerticalAligment {
    /// Align top.
    Top,
    /// Align middle.
    Middle,
    /// Align bottom.
    Bottom,
}

impl TextHorizontalAlignment {
    fn get_padding_x<T: AsRef<[u8]>>(&self, text: T) -> i32 {
        let len = text.as_ref().len() as i32;

        match *self {
            Self::Left => 0,
            Self::Center => {
                (((CHARS_PER_LINE as f32 - len as f32) / 2.0) * CHAR_WIDTH as f32) as i32
            }
            Self::Right => (CHARS_PER_LINE as i32 - len) * CHAR_WIDTH as i32,
        }
    }
}

impl TextVerticalAligment {
    fn get_padding_y(&self, line_count: usize, line_separation: i32) -> i32 {
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

/// A text drawing helper.
///
/// # Example
///
/// ```no_run
/// use wasm4_sx::*;
///
/// #[no_mangle]
/// fn update() {
///     Text::new("Top left")
///         .with_horizontal_alignment(TextHorizontalAlignment::Left)
///         .with_vertical_alignment(TextVerticalAligment::Top)
///         .draw();
///
///     Text::new("Middle center")
///         .with_horizontal_alignment(TextHorizontalAlignment::Center)
///         .with_vertical_alignment(TextVerticalAligment::Middle)
///         .draw();
///
///     Text::new("Bottom right")
///         .with_horizontal_alignment(TextHorizontalAlignment::Right)
///         .with_vertical_alignment(TextVerticalAligment::Bottom)
///         .draw();
/// }
/// ```
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
    /// Build a new text drawing helper.
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

    /// Set the X coordinate.
    pub fn with_x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    /// Set the Y coordinate.
    pub fn with_y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    /// Set the X padding.
    pub fn with_padding_x(mut self, padding_x: i32) -> Self {
        self.padding_x = padding_x;
        self
    }

    /// Set the Y padding.
    pub fn with_padding_y(mut self, padding_y: i32) -> Self {
        self.padding_y = padding_y;
        self
    }

    /// Set the horizontal alignment.
    pub fn with_horizontal_alignment(mut self, alignment: TextHorizontalAlignment) -> Self {
        self.horizontal_alignment = Some(alignment);
        self
    }

    /// Set the vertical alignment.
    pub fn with_vertical_alignment(mut self, alignment: TextVerticalAligment) -> Self {
        self.vertical_alignment = Some(alignment);
        self
    }

    /// Set the line separation value.
    pub fn with_line_separation(mut self, line_separation: i32) -> Self {
        self.line_separation = line_separation;
        self
    }

    /// Draw the text.
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

    fn compute_draw_coordinates<U: AsRef<[u8]>>(
        &self,
        line_index: usize,
        line_count: usize,
        line: U,
    ) -> (i32, i32) {
        let x = self.horizontal_padding(line.as_ref()) + self.x;
        let y = self.vertical_padding(line_count)
            + self.y
            + (CHAR_WIDTH as i32 + self.line_separation) * line_index as i32;

        (x, y)
    }

    fn draw_line<U: AsRef<[u8]>>(&self, line_index: usize, line_count: usize, line: U) {
        let (x, y) = self.compute_draw_coordinates(line_index, line_count, line.as_ref());
        text(line.as_ref(), x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_aligment() {
        let (x, y) = Text::new("Hello")
            .with_horizontal_alignment(TextHorizontalAlignment::Left)
            .compute_draw_coordinates(0, 1, "Hello");

        assert_eq!((x, y), (0, 0));

        let (x, y) = Text::new("Hello")
            .with_horizontal_alignment(TextHorizontalAlignment::Center)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 5 * 8) / 2 = 60
        assert_eq!((x, y), (60, 0));

        let (x, y) = Text::new("Hello")
            .with_horizontal_alignment(TextHorizontalAlignment::Center)
            .with_padding_x(1)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 5 * 8) / 2 + 1 = 61
        assert_eq!((x, y), (61, 0));

        let (x, y) = Text::new("Hello")
            .with_horizontal_alignment(TextHorizontalAlignment::Center)
            .with_padding_x(-1)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 5 * 8) / 2 - 1 = 59
        assert_eq!((x, y), (59, 0));

        let (x, y) = Text::new("Hello")
            .with_horizontal_alignment(TextHorizontalAlignment::Right)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 5 * 8) = 120
        assert_eq!((x, y), (120, 0));

        let (x, y) = Text::new("Hello")
            .with_horizontal_alignment(TextHorizontalAlignment::Right)
            .with_padding_x(1)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 5 * 8) + 1 = 121
        assert_eq!((x, y), (121, 0));
    }

    #[test]
    fn vertical_alignment() {
        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Top)
            .compute_draw_coordinates(0, 1, "Hello");

        assert_eq!((x, y), (0, 0));

        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Middle)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 8) / 2 = 76
        assert_eq!((x, y), (0, 76));

        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Middle)
            .with_padding_y(1)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 8) / 2 + 1 = 77
        assert_eq!((x, y), (0, 77));

        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Middle)
            .with_padding_y(-1)
            .compute_draw_coordinates(0, 1, "Hello");

        // (160 - 8) / 2 - 1 = 75
        assert_eq!((x, y), (0, 75));

        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Bottom)
            .compute_draw_coordinates(0, 1, "Hello");

        // 160 - 8 = 152
        assert_eq!((x, y), (0, 152));

        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Bottom)
            .with_padding_y(1)
            .compute_draw_coordinates(0, 1, "Hello");

        // 160 - 8 + 1 = 153
        assert_eq!((x, y), (0, 153));

        let (x, y) = Text::new("Hello")
            .with_vertical_alignment(TextVerticalAligment::Bottom)
            .with_padding_y(-1)
            .compute_draw_coordinates(0, 1, "Hello");

        // 160 - 8 - 1 = 151
        assert_eq!((x, y), (0, 151));
    }
}
