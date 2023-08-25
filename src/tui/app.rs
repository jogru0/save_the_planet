use std::{collections::HashSet, error};

use crate::world::{Key, World};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync>>;

/// Application.
#[derive(Debug)]
pub struct TuiState {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub pressed_keys: HashSet<Key>,
    pub world: World,
}

impl Default for TuiState {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            pressed_keys: Default::default(),
            world: World::new(),
        }
    }
}

impl TuiState {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
