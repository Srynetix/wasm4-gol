use wasm4_sx::rand_f64;

/// Cell state.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    /// Alive.
    Alive,
    /// Dead.
    Dead,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Dead
    }
}

/// A game cell.
#[derive(Clone, Copy)]
pub struct GameCell {
    /// Cell state.
    pub state: CellState,
}

impl GameCell {
    /// Build a new game cell.
    pub const fn new() -> Self {
        Self {
            state: CellState::Dead,
        }
    }

    /// Randomize the cell state using a probability value (between 0 and 1).
    pub fn randomize(&mut self, alive_probability: f64) {
        self.state = if rand_f64() < alive_probability {
            CellState::Alive
        } else {
            CellState::Dead
        };
    }
}
