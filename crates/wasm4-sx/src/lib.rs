#![no_std]
#![warn(clippy::all)]
#![deny(missing_docs)]

//! wasm4-sx - Opinionated wrapper around WASM-4
//!
//! > Tired of accessing raw pointers and adding "unsafe" everywhere?
//! > Use abstractions, now!
//!
//! ```no_run
//! use wasm4_sx::*;
//!
//! #[no_mangle]
//! fn start() {
//!     // Let's change the palette!
//!     Engine::palette().set(
//!         Palette::new([
//!             Color::new(0, 0, 0),
//!             Color::new(0, 0, 127),
//!             Color::new(0, 127, 127),
//!             Color::new(127, 127, 127),
//!         ])
//!     )
//! }
//!
//! #[no_mangle]
//! fn update() {
//!     Engine::run_frame(|ctx| {
//!         // Let's change draw colors, safely!
//!         Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P1);
//!         Engine::draw_colors().set_index(DrawColorsIndex::I2, PaletteColor::Transparent);
//!
//!         // Let's check if the X button is pressed on gamepad #1
//!         if ctx.gamepad(GamepadIndex::I1).is_button_pressed(GamepadButton::X) {
//!             Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P2);
//!         }
//!
//!         // Let's check if the mouse left-click was just pressed
//!         if ctx.mouse().is_button_just_pressed(MouseButton::Left) {
//!             Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P3);
//!         }
//!     });
//! }
//! ```

mod cell;
mod color;
mod draw_colors;
mod engine;
mod gamepad;
mod mouse;
mod palette;
mod screen;
mod text;
mod vec2;

pub use cell::W4RefCell;
pub use color::Color;
pub use draw_colors::{DrawColors, DrawColorsBuilder, DrawColorsIndex, PaletteColor};
pub use engine::{Engine, FrameContext};
pub use gamepad::{GamepadButton, GamepadIndex, GamepadState};
pub use mouse::{MouseButton, MouseState};
pub use palette::Palette;
pub use text::{Text, TextHorizontalAlignment, TextVerticalAligment};
pub use vec2::Vec2;

// Reexports
pub use const_str;
pub use fastrand;
pub use wasm4_sys as wasm4;
