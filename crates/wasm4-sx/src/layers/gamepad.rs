use crate::{
    cell::W4RefCell,
    wasm4::{
        BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, GAMEPAD1, GAMEPAD2,
        GAMEPAD3, GAMEPAD4,
    },
};

static PREVIOUS_STATE: W4RefCell<[u8; 4]> = W4RefCell::new([0; 4]);

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum GamepadIndex {
    I1,
    I2,
    I3,
    I4,
}

impl GamepadIndex {
    pub fn all() -> [GamepadIndex; 4] {
        [Self::I1, Self::I2, Self::I3, Self::I4]
    }
}

pub enum GamepadButton {
    X,
    Z,
    Left,
    Right,
    Up,
    Down,
}

impl From<u8> for GamepadButton {
    fn from(value: u8) -> Self {
        match value {
            BUTTON_1 => Self::X,
            BUTTON_2 => Self::Z,
            BUTTON_LEFT => Self::Left,
            BUTTON_RIGHT => Self::Right,
            BUTTON_UP => Self::Up,
            BUTTON_DOWN => Self::Down,
            _ => panic!("unknown gamepad button: {value}"),
        }
    }
}

impl From<GamepadButton> for u8 {
    fn from(value: GamepadButton) -> Self {
        match value {
            GamepadButton::X => BUTTON_1,
            GamepadButton::Z => BUTTON_2,
            GamepadButton::Left => BUTTON_LEFT,
            GamepadButton::Right => BUTTON_RIGHT,
            GamepadButton::Up => BUTTON_UP,
            GamepadButton::Down => BUTTON_DOWN,
        }
    }
}

#[derive(Default, Debug)]
pub struct GamepadState {
    just_pressed: u8,
    pressed: u8,
}

impl GamepadState {
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        self.pressed & u8::from(button) != 0
    }

    pub fn is_button_just_pressed(&self, button: GamepadButton) -> bool {
        self.just_pressed & u8::from(button) != 0
    }
}

pub struct GlobalGamepads;

impl GlobalGamepads {
    pub fn get_gamepad(index: GamepadIndex) -> GamepadState {
        Self::get_gamepad_inner(
            read_raw_gamepad_value(index),
            PREVIOUS_STATE.borrow()[index as usize],
        )
    }

    pub fn get_all_gamepads() -> [GamepadState; 4] {
        [
            Self::get_gamepad(GamepadIndex::I1),
            Self::get_gamepad(GamepadIndex::I2),
            Self::get_gamepad(GamepadIndex::I3),
            Self::get_gamepad(GamepadIndex::I4),
        ]
    }

    fn get_gamepad_inner(gamepad_handle: u8, previous_state: u8) -> GamepadState {
        let pressed_this_frame = gamepad_handle & (gamepad_handle ^ previous_state);

        GamepadState {
            just_pressed: pressed_this_frame,
            pressed: gamepad_handle,
        }
    }

    fn update_previous_state(index: GamepadIndex) {
        PREVIOUS_STATE.borrow_mut()[index as usize] = read_raw_gamepad_value(index);
    }

    pub fn tick_frame_end() {
        GamepadIndex::all()
            .iter()
            .for_each(|index| Self::update_previous_state(*index))
    }
}

fn read_raw_gamepad_value(index: GamepadIndex) -> u8 {
    // Safety: gamepad locations are hard-coded, and only read.
    unsafe {
        match index {
            GamepadIndex::I1 => *GAMEPAD1,
            GamepadIndex::I2 => *GAMEPAD2,
            GamepadIndex::I3 => *GAMEPAD3,
            GamepadIndex::I4 => *GAMEPAD4,
        }
    }
}
