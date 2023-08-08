use crate::wasm4::PALETTE;
use crate::Color;

/// Palette index.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PaletteIndex {
    /// First palette index.
    P1 = 0,
    /// Second palette index.
    P2,
    /// Third palette index.
    P3,
    /// Fourth palette index.
    P4,
}

/// Color palette.
pub struct Palette([Color; 4]);

impl Palette {
    /// Build a new color palette.
    pub fn new(colors: [Color; 4]) -> Self {
        Self(colors)
    }

    /// Set a specific palette color.
    pub fn set_color(&mut self, index: PaletteIndex, value: Color) {
        self.0[index as usize] = value;
    }
}

impl From<Palette> for [u32; 4] {
    fn from(value: Palette) -> Self {
        value.0.map(Into::into)
    }
}

impl From<[u32; 4]> for Palette {
    fn from(value: [u32; 4]) -> Self {
        Palette(value.map(Into::into))
    }
}

/// System palette.
pub struct SystemPalette {
    _private: (),
}

impl SystemPalette {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    /// Get the system palette.
    pub fn get(&self) -> Palette {
        read_system_palette()
    }

    /// Set the system palette.
    pub fn set(&self, palette: Palette) {
        write_system_palette(palette)
    }
}

fn read_system_palette() -> Palette {
    // Safety: palette location is hard-coded.
    unsafe { (*PALETTE).into() }
}

fn write_system_palette(palette: Palette) {
    // Safety: the Palette struct guarantee a valid palette value.
    unsafe { *PALETTE = palette.into() }
}
