#![no_std]
#![warn(clippy::all)]

mod logic;

use logic::{randomize_grid, run_game_frame};
use wasm4_sx::*;

#[no_mangle]
fn start() {
    randomize_grid(0.5);
}

#[no_mangle]
fn update() {
    Engine::run_frame(|ctx| run_game_frame(&ctx));
}
