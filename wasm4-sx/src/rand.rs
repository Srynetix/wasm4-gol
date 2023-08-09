use core::ops::RangeBounds;

use fastrand::Rng;

use crate::W4RefCell;

static GLOBAL_RNG: W4RefCell<Option<Rng>> = W4RefCell::new(None);

fn initialize() {
    *GLOBAL_RNG.borrow_mut() = Some(Rng::with_seed(123456789));
}

fn ensure_initialized() {
    if GLOBAL_RNG.borrow().is_none() {
        initialize();
    }
}

/// Generate a random u8.
pub fn rand_u8(range: impl RangeBounds<u8>) -> u8 {
    ensure_initialized();
    GLOBAL_RNG.borrow_mut().as_mut().unwrap().u8(range)
}

/// Generate a random f64.
pub fn rand_f64() -> f64 {
    ensure_initialized();
    GLOBAL_RNG.borrow_mut().as_mut().unwrap().f64()
}

/// Reseed the RNG.
pub fn rand_reseed(seed: u64) {
    ensure_initialized();
    GLOBAL_RNG.borrow_mut().as_mut().unwrap().seed(seed)
}
