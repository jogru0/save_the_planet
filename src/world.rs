use crate::duration::Duration;

use self::{cards::Cards, message::Messages, quantity::Quantity};

mod rate;

mod quantity;

mod cards;

mod message;

pub struct World {
    cards: Cards,
    total_ticks: Duration,
    messages: Messages,
}

impl World {
    pub fn get_total_ticks(&self) -> Duration {
        self.total_ticks
    }

    pub fn new() -> Self {
        Self {
            cards: Cards::new(),
            messages: Messages::new(),
            total_ticks: Duration::INSTANT,
        }
    }

    pub fn simulate(&mut self, total_ticks: Duration) {
        assert!(self.total_ticks <= total_ticks);
        let delta = total_ticks - self.total_ticks;

        self.total_ticks += delta;
        self.simulate_cards(delta);
        self.messages.simulate(delta);
    }
}

pub mod render;
