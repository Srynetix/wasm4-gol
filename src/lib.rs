#![no_std]
#![warn(clippy::all)]
#![deny(missing_docs)]

//! Game of Life for WASM-4.

mod game;
mod game_cell;

use game::Game;
use wasm4_sx::*;

#[cfg(test)]
extern crate wasm4_stubs;

#[no_mangle]
fn start() {
    Game::randomize_grid(0.5);
}

#[no_mangle]
fn update() {
    Engine::run_frame(|ctx| Game::run_game_frame(&ctx));
}

wasm4_sx::setup_panic_handler_w4!();
