use crate::{
    grid::{Cell, MutGridView},
    world::render::{CHARS_CARD, LINES_MAIN_FRAME_CONTENT},
};

use self::{
    abstract_card::AbstractCard, activism::Activism, milestones::Milestones, research::Research,
    staff::Staff,
};

use super::{duration::Duration, quantity::Quantity, Input, World};

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
impl Card {
    fn simulate(self, world: &mut World, delta: Duration) {
        match self {
            Card::CO2 => world.simulate_card_activism(delta),
            Card::Milestones => world.simulate_card_milestones(delta),
            Card::Research => world.simulate_card_research(delta),
            Card::Staff => world.simulate_card_staff(delta),
        }
    }

    fn render(self, world: &mut World, input: &Input, view: MutGridView<'_, Cell>) {
        match self {
            Card::CO2 => world.render_card_activism(input, view),
            Card::Milestones => world.render_card_milestones(input, view),
            Card::Research => world.render_card_research(input, view),
            Card::Staff => world.render_card_staff(input, view),
        }
    }
}

const ALL_CARDS: [Card; 4] = [Card::CO2, Card::Milestones, Card::Research, Card::Staff];

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

        for card in ALL_CARDS {
            card.simulate(self, delta);
        }
    }

    pub fn render_card(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME_CONTENT);
        assert!(CHARS_CARD <= view.width());
        self.cards.selected.render(self, input, view)
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
