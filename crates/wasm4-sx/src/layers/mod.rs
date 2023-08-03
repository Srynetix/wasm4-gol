mod color;
mod draw_colors;
mod gamepad;
mod mouse;
mod palette;
mod screen;

pub use color::Color;
pub use draw_colors::{DrawColors, DrawColorsIndex, PaletteColor};
pub use gamepad::{GamepadButton, GamepadIndex, GamepadState, GlobalGamepads};
pub use mouse::{GlobalMouse, MouseButton, MouseState};
pub use palette::Palette;
pub use screen::Screen;

pub(crate) use draw_colors::GlobalDrawColors;
pub(crate) use palette::GlobalPalette;
