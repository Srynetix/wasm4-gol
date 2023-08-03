use core::sync::atomic::{AtomicBool, Ordering};

use wasm4_sx::{
    const_str::concat_bytes,
    fastrand,
    wasm4::{rect, SCREEN_SIZE},
    DrawColorsIndex, Engine, FrameContext, GamepadButton, GamepadIndex, MouseButton, PaletteColor,
    Text, TextHorizontalAlignment, TextVerticalAligment, W4RefCell,
};

const CELL_SIZE: u32 = 2;
const GRID_WIDTH: u32 = SCREEN_SIZE / CELL_SIZE;
const GRID_HEIGHT: u32 = SCREEN_SIZE / CELL_SIZE;
const GRID_CELL_COUNT: usize = (GRID_WIDTH * GRID_HEIGHT) as usize;
const DRAW_INSTRUCTIONS_FOR_SECONDS: u64 = 10;

static GRID_BUFFER_FRONT: W4RefCell<[GameCell; GRID_CELL_COUNT]> =
    W4RefCell::new([GameCell::new(); GRID_CELL_COUNT]);
static GRID_BUFFER_BACK: W4RefCell<[GameCell; GRID_CELL_COUNT]> =
    W4RefCell::new([GameCell::new(); GRID_CELL_COUNT]);
static SIMULATION_RUNNING: AtomicBool = AtomicBool::new(true);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Dead
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameCell {
    state: CellState,
    age: u8,
}

impl GameCell {
    pub const fn new() -> Self {
        Self {
            state: CellState::Dead,
            age: 0,
        }
    }

    pub fn randomize(&mut self, alive_probability: f64) {
        self.age = 0;
        self.state = if fastrand::f64() < alive_probability {
            CellState::Alive
        } else {
            CellState::Dead
        };
    }
}

pub fn randomize_grid(alive_probability: f64) {
    GRID_BUFFER_BACK
        .borrow_mut()
        .iter_mut()
        .for_each(|c| c.randomize(alive_probability));

    swap_buffers();
}

pub fn run_game_frame(ctx: &FrameContext) {
    interact_grid(ctx);
    step_grid();
    render_grid();

    // Hide after 10 seconds
    if Engine::frame_count() < Engine::FPS * DRAW_INSTRUCTIONS_FOR_SECONDS {
        draw_instructions();
    }
}

fn swap_buffers() {
    GRID_BUFFER_FRONT
        .borrow_mut()
        .iter_mut()
        .zip(GRID_BUFFER_BACK.borrow().iter())
        .for_each(|(dest, src)| {
            *dest = *src;
        });
}

fn interact_grid(ctx: &FrameContext) {
    let (mouse_x, mouse_y) = ctx.mouse().position();
    let cell_x = (mouse_x.max(0) as u32 / CELL_SIZE).min(GRID_WIDTH - 1);
    let cell_y = (mouse_y.max(0) as u32 / CELL_SIZE).min(GRID_HEIGHT - 1);

    if ctx.mouse().is_button_pressed(MouseButton::Left) {
        set_cell_state(cell_x, cell_y, CellState::Alive);
    } else if ctx.mouse().is_button_pressed(MouseButton::Right) {
        set_cell_state(cell_x, cell_y, CellState::Dead);
    }

    if ctx
        .gamepad(GamepadIndex::I1)
        .is_button_just_pressed(GamepadButton::X)
    {
        set_simulation_running_state(!get_simulation_running_state());
    }

    if ctx
        .gamepad(GamepadIndex::I1)
        .is_button_just_pressed(GamepadButton::Z)
    {
        clear_grid();
    }
}

fn set_cell_state(x: u32, y: u32, state: CellState) {
    let idx = xy_to_index(x, y);

    GRID_BUFFER_BACK.borrow_mut()[idx].state = state;
    GRID_BUFFER_FRONT.borrow_mut()[idx].state = state;
}

fn step_grid() {
    if !get_simulation_running_state() {
        return;
    }

    for (idx, cell) in GRID_BUFFER_FRONT.borrow().iter().enumerate() {
        let (x, y) = index_to_xy(idx);
        let neighbors = alive_neighbors_count(&*GRID_BUFFER_FRONT.borrow(), x, y);

        GRID_BUFFER_BACK.borrow_mut()[idx].state = match cell.state {
            CellState::Alive if neighbors == 2 || neighbors == 3 => CellState::Alive,
            CellState::Dead if neighbors == 3 => CellState::Alive,
            _ => CellState::Dead,
        };
    }

    swap_buffers()
}

fn render_grid() {
    for (idx, cell) in GRID_BUFFER_FRONT.borrow().iter().enumerate() {
        let (x, y) = index_to_xy(idx);

        if cell.state == CellState::Dead {
            Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P4)
        } else {
            Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P1)
        }

        rect(
            (x * CELL_SIZE) as i32,
            (y * CELL_SIZE) as i32,
            CELL_SIZE,
            CELL_SIZE,
        )
    }
}

fn clear_grid() {
    GRID_BUFFER_BACK.borrow_mut().iter_mut().for_each(|c| {
        c.state = CellState::Dead;
        c.age = 0;
    });

    swap_buffers();
}

fn alive_neighbors_count(buffer: &[GameCell], x: u32, y: u32) -> u32 {
    [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    .map(|offset| wrap_position((x, y), *offset))
    .filter(|(x, y)| cell_is_alive(buffer, *x, *y))
    .count() as u32
}

fn wrap_position((x, y): (u32, u32), (offset_x, offset_y): (i32, i32)) -> (u32, u32) {
    let x = x as i32;
    let y = y as i32;

    (
        (x + offset_x).rem_euclid(GRID_WIDTH as i32) as u32,
        (y + offset_y).rem_euclid(GRID_HEIGHT as i32) as u32,
    )
}

fn cell_is_alive(buffer: &[GameCell], x: u32, y: u32) -> bool {
    let idx = xy_to_index(x, y);
    buffer[idx].state == CellState::Alive
}

fn xy_to_index(x: u32, y: u32) -> usize {
    (x + y * GRID_WIDTH) as usize
}

fn index_to_xy(index: usize) -> (u32, u32) {
    ((index as u32) % GRID_WIDTH, (index as u32) / GRID_WIDTH)
}

fn set_simulation_running_state(value: bool) {
    SIMULATION_RUNNING.store(value, Ordering::Relaxed)
}

fn get_simulation_running_state() -> bool {
    SIMULATION_RUNNING.load(Ordering::Relaxed)
}

fn draw_instructions() {
    Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P2);
    Engine::draw_colors().set_index(DrawColorsIndex::I2, PaletteColor::P4);

    Text::new("Game of Life\nby @Srynetix")
        .with_horizontal_alignment(TextHorizontalAlignment::Center)
        .with_vertical_alignment(TextVerticalAligment::Top)
        .with_line_separation(2)
        .with_padding_y(2)
        .draw();

    Text::new("Have fun!")
        .with_horizontal_alignment(TextHorizontalAlignment::Center)
        .with_vertical_alignment(TextVerticalAligment::Middle)
        .draw();

    Text::new(concat_bytes!(
        b"\x80 to pause/resume\n",
        b"\x81 to clear grid\n",
        b"Left-click to draw\n",
        b"Right-click to erase"
    ))
    .with_horizontal_alignment(TextHorizontalAlignment::Center)
    .with_vertical_alignment(TextVerticalAligment::Bottom)
    .with_line_separation(2)
    .with_padding_y(-2)
    .draw();

    Engine::draw_colors().set_index(DrawColorsIndex::I2, PaletteColor::Transparent);
}
