use self::{
    abstract_card::AbstractCard, activism::Activism, milestones::Milestones, research::Research,
    staff::Staff,
};

use super::{duration::Duration, quantity::Quantity, World};

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

mod abstract_card;

#[derive(Debug)]
pub struct Cards {
    pub selected: Card,
    co2: Activism,
    milestones: Milestones,
    research: Research,
    staff: Staff,
}

impl World {
    pub fn simulate_cards(&mut self, delta: Duration) {
        if !self.cards.milestones.is_visible()
            && self.cards.co2.emission_balance.balance() >= (Quantity::new(1000))
        {
            self.cards.milestones.discover();
        }

        self.simulate_card_co2(delta);
    }
}

impl Cards {
    pub fn new() -> Cards {
        Self {
            selected: Card::CO2,
            co2: Activism::new(),
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
            Card::CO2 => &self.co2,
            Card::Milestones => &self.milestones,
            Card::Research => &self.research,
            Card::Staff => &self.staff,
        }
    }
}
