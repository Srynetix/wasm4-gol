use core::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

/// A "safe" global RefCell to use in the WASM-4 context
pub struct W4RefCell<T>(RefCell<T>);

impl<T> W4RefCell<T> {
    /// Build a new RefCell.
    pub const fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }
}

impl<T> Deref for W4RefCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for W4RefCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Safety: Only valid in the WASM-4 context.
unsafe impl<T> Sync for W4RefCell<T> {}
