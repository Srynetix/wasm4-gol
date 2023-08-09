use core::sync::atomic::{AtomicU8, Ordering};

use crate::wasm4::{MOUSE_BUTTONS, MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT, MOUSE_X, MOUSE_Y};

static PREVIOUS_STATE: AtomicU8 = AtomicU8::new(0);

/// Mouse button.
#[derive(Clone, Copy)]
pub enum MouseButton {
    /// Left button.
    Left,
    /// Right button.
    Right,
    /// Middle button.
    Middle,
}

impl From<u8> for MouseButton {
    fn from(value: u8) -> Self {
        match value {
            MOUSE_LEFT => Self::Left,
            MOUSE_RIGHT => Self::Right,
            MOUSE_MIDDLE => Self::Middle,
            _ => panic!("unknown mouse button: {value}"),
        }
    }
}

impl From<MouseButton> for u8 {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => MOUSE_LEFT,
            MouseButton::Right => MOUSE_RIGHT,
            MouseButton::Middle => MOUSE_MIDDLE,
        }
    }
}

/// Mouse state.
#[derive(Default)]
pub struct MouseState {
    x: i16,
    y: i16,
    just_pressed: u8,
    pressed: u8,
}

impl MouseState {
    /// X coordinate.
    pub fn x(&self) -> i16 {
        self.x
    }

    /// Y coordinate.
    pub fn y(&self) -> i16 {
        self.y
    }

    /// Mouse position.
    pub fn position(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    /// Check if a button is pressed.
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed & u8::from(button) != 0
    }

    /// Check if a button was just pressed.
    pub fn is_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed & u8::from(button) != 0
    }
}

pub struct GlobalMouse;

impl GlobalMouse {
    pub fn get() -> MouseState {
        let current = read_raw_mouse_buttons_value();
        let previous = PREVIOUS_STATE.load(Ordering::Relaxed);
        let pressed_this_frame = current & (current ^ previous);
        let (x, y) = read_raw_mouse_position();

        MouseState {
            x,
            y,
            just_pressed: pressed_this_frame,
            pressed: current,
        }
    }

    pub fn tick_frame_end() {
        let raw = read_raw_mouse_buttons_value();
        PREVIOUS_STATE.store(raw, Ordering::Relaxed);
    }
}

fn read_raw_mouse_buttons_value() -> u8 {
    // Safety: mouse buttons location is hard-coded, and only read.
    unsafe { *MOUSE_BUTTONS }
}

fn read_raw_mouse_position() -> (i16, i16) {
    // Safety: mouse coordinate locations are hard-coded, and only read.
    unsafe { (*MOUSE_X, *MOUSE_Y) }
}
