use crate::wasm4::DRAW_COLORS;

/// Palette color.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PaletteColor {
    /// Transparent.
    Transparent = 0,
    /// First palette color.
    P1,
    /// Second palette color.
    P2,
    /// Third palette color.
    P3,
    /// Fourth palette color.
    P4,
}

/// Draw colors index.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum DrawColorsIndex {
    /// First draw color.
    I1 = 0,
    /// Second draw color.
    I2,
    /// Third draw color.
    I3,
    /// Fourth draw color.
    I4,
}

/// Draw colors.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DrawColors([u8; 4]);

impl DrawColors {
    /// Create a new draw colors builder.
    pub fn builder() -> DrawColorsBuilder {
        DrawColorsBuilder::new()
    }
}

/// Draw colors builder.
#[derive(Default)]
pub struct DrawColorsBuilder([u8; 4]);

impl DrawColorsBuilder {
    /// Create a new draw colors builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a specific palette color for a specific draw color index.
    pub fn with_index(mut self, index: DrawColorsIndex, value: PaletteColor) -> Self {
        self.0[index as usize] = value as u8;
        self
    }

    /// Build draw colors.
    pub fn build(self) -> DrawColors {
        DrawColors(self.0)
    }
}

/// System draw colors.
pub struct SystemDrawColors {
    _private: (),
}

impl SystemDrawColors {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    /// Set system draw colors.
    pub fn set(&self, colors: DrawColors) {
        write_system_draw_colors(colors)
    }

    /// Reset system draw colors.
    pub fn reset(&self) {
        self.set(DrawColors::default())
    }

    /// Set a specific system draw color using color index and palette index.
    pub fn set_index(&self, index: DrawColorsIndex, value: PaletteColor) {
        let mut existing = self.get();
        existing.0[index as usize] = value as u8;

        self.set(existing);
    }

    /// Get system draw colors.
    pub fn get(&self) -> DrawColors {
        read_system_draw_colors()
    }
}

impl From<DrawColors> for u16 {
    fn from(value: DrawColors) -> Self {
        value.0[0] as u16
            + shift_u16(value.0[1], 4)
            + shift_u16(value.0[2], 8)
            + shift_u16(value.0[3], 12)
    }
}

impl From<u16> for DrawColors {
    fn from(value: u16) -> Self {
        let i3 = ((value & 0xf000) >> 12) as u8;
        let i2 = ((value & 0x0f00) >> 8) as u8;
        let i1 = ((value & 0x00f0) >> 4) as u8;
        let i0 = (value & 0x000f) as u8;

        Self([i0, i1, i2, i3])
    }
}

fn shift_u16(v: u8, amount: u16) -> u16 {
    (v as u16) << amount
}

fn read_system_draw_colors() -> DrawColors {
    // Safety: draw colors location is hard-coded.
    unsafe { (*DRAW_COLORS).into() }
}

fn write_system_draw_colors(colors: DrawColors) {
    // Safety: the DrawColors struct guarantee a valid colors value.
    unsafe { *DRAW_COLORS = u16::from(colors) }
}
