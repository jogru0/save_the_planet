use std::error;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;

use self::cards::Cards;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default().with_selected(Some(0)),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub mod cards;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u128,
    pub buyers: u128,

    pub cards: Cards,

    pub old_key: Option<KeyCode>,
    pub key: Option<KeyCode>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            buyers: 0,
            cards: Cards::new(),
            key: None,
            old_key: None,
        }
    }
}

impl App {
    pub fn check_key(&mut self, key: KeyCode) -> bool {
        if self.key == Some(key) {
            self.key = None;
            true
        } else {
            false
        }
    }

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

    pub fn add_buyer(&mut self) {
        if self.counter >= 20 {
            self.counter -= 20;
            self.buyers += 1;
        }
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

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> AppResult<()> {
        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Esc | KeyCode::Char('q') => {
                self.quit();
            }

            code => {
                self.key = Some(code);
            }
        }
        Ok(())
    }

    pub fn simulate(&mut self, seconds: u64) {
        self.cards.flyer.saved_co2 += seconds as u128;

        self.counter += self.buyers * seconds as u128;

        if self.buyers >= 1000 {
            self.running = false;
        }
    }
}
