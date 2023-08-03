#![no_std]
#![warn(clippy::all)]

mod logic;

use logic::{draw_instructions, interact_grid, randomize_grid, render_grid, step_grid};
use wasm4_sx::*;

#[no_mangle]
fn start() {
    randomize_grid(0.5);
}

#[no_mangle]
fn update() {
    Engine::run_frame(|ctx| {
        interact_grid(&ctx);
        step_grid();
        render_grid();

        // Hide after 10 seconds
        if Engine::frame_count() < Engine::FPS * 10 {
            draw_instructions();
        }
    });
}
