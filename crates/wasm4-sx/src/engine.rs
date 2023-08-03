use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use super::layers::{
    GamepadIndex, GamepadState, GlobalDrawColors, GlobalGamepads, GlobalMouse, GlobalPalette,
    MouseState, Screen,
};

static FRAME_SKIP: AtomicU32 = AtomicU32::new(0);
static FRAME_COUNT: AtomicU64 = AtomicU64::new(0);

pub struct Engine;

impl Engine {
    pub const FPS: u64 = 60;

    pub fn set_frame_skip(value: u32) {
        FRAME_SKIP.store(value, Ordering::Relaxed);
    }

    pub fn run_frame<F: Fn(FrameContext)>(func: F) {
        let current_frame = Self::frame_count();
        let frame_skip = Self::frame_skipped();

        if frame_skip == 0 || frame_skip > 0 && current_frame % frame_skip as u64 == 0 {
            func(FrameContext::new());
        }

        FRAME_COUNT.store(current_frame + 1, Ordering::Relaxed);
        Engine::tick_frame_end();
    }

    pub fn frame_count() -> u64 {
        FRAME_COUNT.load(Ordering::Relaxed)
    }

    pub fn frame_skipped() -> u32 {
        FRAME_SKIP.load(Ordering::Relaxed)
    }

    pub fn palette() -> GlobalPalette {
        GlobalPalette::new()
    }

    pub fn draw_colors() -> GlobalDrawColors {
        GlobalDrawColors::new()
    }

    fn tick_frame_end() {
        GlobalMouse::tick_frame_end();
        GlobalGamepads::tick_frame_end();
    }
}

pub struct FrameContext {
    gamepads: [GamepadState; 4],
    mouse: MouseState,
}

impl FrameContext {
    pub fn gamepads(&self) -> &[GamepadState; 4] {
        &self.gamepads
    }

    pub fn gamepad(&self, index: GamepadIndex) -> &GamepadState {
        &self.gamepads[index as usize]
    }

    pub fn mouse(&self) -> &MouseState {
        &self.mouse
    }

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
