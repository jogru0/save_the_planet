use crate::{
    duration::Duration,
    grid::{Cell, MutGridView},
    input::Input,
    world::render::{CHARS_CARD, LINES_MAIN_FRAME_CONTENT},
};

use self::{
    abstract_card::AbstractCard, activism::Activism, milestones::Milestones, research::Research,
    staff::Staff,
};

use super::{quantity::Quantity, World};

mod activism;
mod milestones;
mod research;
mod staff;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Card {
    CO2,
    Milestones,
    Research,
    Staff,
}

const ALL_CARDS: [Card; 4] = [Card::CO2, Card::Milestones, Card::Research, Card::Staff];

mod abstract_card;

pub struct Cards {
    pub selected: Card,
    activism: Activism,
    milestones: Milestones,
    research: Research,
    staff: Staff,
}

impl World {
    pub fn simulate_cards(&mut self, delta: Duration) {
        if !self.cards.milestones.is_visible()
            && self.cards.activism.emission_balance.balance() >= (Quantity::new(1000))
        {
            self.cards.milestones.discover();
        }

        for card in ALL_CARDS {
            match card {
                Card::CO2 => self.simulate_card_activism(delta),
                Card::Milestones => self.simulate_card_milestones(delta),
                Card::Research => self.simulate_card_research(delta),
                Card::Staff => self.simulate_card_staff(delta),
            }
        }
    }

    pub fn render_card(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME_CONTENT);
        assert!(CHARS_CARD <= view.width());
        match self.cards.selected {
            Card::CO2 => self.render_card_activism(input, view),
            Card::Milestones => self.render_card_milestones(input, view),
            Card::Research => self.render_card_research(input, view),
            Card::Staff => self.render_card_staff(input, view),
        }
    }
}

impl Cards {
    pub fn new() -> Cards {
        Self {
            selected: Card::CO2,
            activism: Activism::new(),
            milestones: Milestones::new(),
            staff: Staff::new(),
            research: Research::new(),
        }
    }

    pub fn available_cards(&self) -> Vec<Card> {
        [Card::CO2, Card::Research, Card::Milestones, Card::Staff]
            .into_iter()
            .filter(|card| self.get_card(*card).is_visible())
            .collect()
    }

    pub fn get_card(&self, card: Card) -> &dyn AbstractCard {
        match card {
            Card::CO2 => &self.activism,
            Card::Milestones => &self.milestones,
            Card::Research => &self.research,
            Card::Staff => &self.staff,
        }
    }
}
