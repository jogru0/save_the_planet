use crate::{
    duration::Duration,
    grid::{Cell, Color, MutGridView},
    input::Input,
    world::{
        quantity::{types::Person, Quantity},
        World,
    },
};

use super::abstract_card::AbstractCard;

impl AbstractCard for Staff {
    fn menu_string(&self) -> String {
        "Staff".into()
    }

    fn color(&self) -> Color {
        Color::WHITE
    }

    fn is_visible(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Staff {
    pub researcher: Quantity<Person>,
    _activists: Quantity<Person>,
}
impl Staff {
    pub fn new() -> Staff {
        Staff {
            researcher: Quantity::default(),
            _activists: Quantity::default(),
        }
    }
}

impl World {
    pub(super) fn render_card_staff(&mut self, _input: &Input, mut _view: MutGridView<'_, Cell>) {}

    pub(super) fn simulate_card_staff(&mut self, _delta: Duration) {}
}
