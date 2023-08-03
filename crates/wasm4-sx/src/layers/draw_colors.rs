use crate::wasm4::DRAW_COLORS;

#[repr(u8)]
pub enum PaletteColor {
    Transparent = 0,
    P1,
    P2,
    P3,
    P4,
}

#[repr(u8)]
pub enum DrawColorsIndex {
    I1 = 0,
    I2,
    I3,
    I4,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct DrawColors([u8; 4]);

impl DrawColors {
    pub fn builder() -> DrawColorsBuilder {
        DrawColorsBuilder::new()
    }
}

#[derive(Default)]
pub struct DrawColorsBuilder([u8; 4]);

impl DrawColorsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_index(mut self, index: DrawColorsIndex, value: PaletteColor) -> Self {
        self.0[index as usize] = value as u8;
        self
    }

    pub fn build(self) -> DrawColors {
        DrawColors(self.0)
    }
}

#[derive(Debug)]
pub struct GlobalDrawColors {
    _private: (),
}

impl GlobalDrawColors {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    pub fn set(&self, colors: DrawColors) {
        write_system_draw_colors(colors)
    }

    pub fn reset(&self) {
        self.set(DrawColors::default())
    }

    pub fn set_index(&self, index: DrawColorsIndex, value: PaletteColor) {
        let mut existing = self.get();
        existing.0[index as usize] = value as u8;

        self.set(existing);
    }

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
