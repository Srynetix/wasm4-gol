use crate::wasm4::PALETTE;

use super::Color;

pub struct Palette([Color; 4]);

impl Palette {
    pub fn new(colors: [Color; 4]) -> Self {
        Self(colors)
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

#[derive(Debug)]
pub struct GlobalPalette {
    _private: (),
}

impl GlobalPalette {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    pub fn get(&self) -> Palette {
        read_system_palette()
    }

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
