/// RGB color with u8 components.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Color {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
}

impl Color {
    /// Build a new color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn shift_u32(value: u8, amount: u32) -> u32 {
        (value as u32) << amount
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = ((value & 0xff0000) >> 16) as u8;
        let g = ((value & 0x00ff00) >> 8) as u8;
        let b = (value & 0x0000ff) as u8;

        Color::new(r, g, b)
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> u32 {
        Color::shift_u32(value.r, 16) + Color::shift_u32(value.g, 8) + value.b as u32
    }
}
