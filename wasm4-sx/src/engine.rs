use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use crate::{
    draw_colors::SystemDrawColors,
    gamepad::{GamepadIndex, GamepadState, GlobalGamepads},
    mouse::{GlobalMouse, MouseState},
    palette::SystemPalette,
    screen::Screen,
};

static FRAME_SKIP: AtomicU32 = AtomicU32::new(0);
static FRAME_COUNT: AtomicU64 = AtomicU64::new(0);

/// Engine: wrapper around direct layers.
pub struct Engine;

impl Engine {
    /// Target frames per second.
    pub const FPS: u64 = 60;

    /// Set frame skip.
    ///
    /// Set to 0 to unset.
    pub fn set_frame_skip(value: u32) {
        FRAME_SKIP.store(value, Ordering::Relaxed);
    }

    /// Execute code for a frame.
    ///
    /// You need to execute your game logic in the `func` closure
    /// to benefit from the gamepad and mouse book-keeping, plus
    /// the frame skip.
    pub fn run_frame<F: Fn(FrameContext)>(func: F) {
        let current_frame = Self::frame_count();
        let frame_skip = Self::frame_skipped();

        if frame_skip == 0 || frame_skip > 0 && current_frame % frame_skip as u64 == 0 {
            func(FrameContext::new());
        }

        FRAME_COUNT.store(current_frame + 1, Ordering::Relaxed);
        Engine::tick_frame_end();
    }

    /// Get the current frame count.
    pub fn frame_count() -> u64 {
        FRAME_COUNT.load(Ordering::Relaxed)
    }

    /// Get the frame skip value.
    pub fn frame_skipped() -> u32 {
        FRAME_SKIP.load(Ordering::Relaxed)
    }

    /// Get the global palette.
    pub fn palette() -> SystemPalette {
        SystemPalette::new()
    }

    /// Get the global draw colors.
    pub fn draw_colors() -> SystemDrawColors {
        SystemDrawColors::new()
    }

    fn tick_frame_end() {
        GlobalMouse::tick_frame_end();
        GlobalGamepads::tick_frame_end();
    }
}

/// Frame context: engine state for a frame.
pub struct FrameContext {
    gamepads: [GamepadState; 4],
    mouse: MouseState,
}

impl FrameContext {
    /// Get the gamepads state.
    pub fn gamepads(&self) -> &[GamepadState; 4] {
        &self.gamepads
    }

    /// Get a specific gamepad state.
    pub fn gamepad(&self, index: GamepadIndex) -> &GamepadState {
        &self.gamepads[index as usize]
    }

    /// Get the mouse state.
    pub fn mouse(&self) -> &MouseState {
        &self.mouse
    }

    /// Get the screen state.
    pub fn screen(&self) -> &Screen {
        Screen::get()
    }

    fn new() -> Self {
        Self {
            gamepads: GlobalGamepads::get_all_gamepads(),
            mouse: GlobalMouse::get(),
        }
    }
}
