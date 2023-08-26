use crate::{
    grid::{Cell, Color, MutGridView},
    world::{
        duration::Duration,
        quantity::{types::Person, Quantity},
        Input, World,
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
    _researcher: Quantity<Person>,
    _activists: Quantity<Person>,
}
impl Staff {
    pub fn new() -> Staff {
        Staff {
            _researcher: Quantity::default(),
            _activists: Quantity::default(),
        }
    }
}

impl World {
    pub(super) fn render_card_staff(&mut self, _input: &Input, mut _view: MutGridView<'_, Cell>) {}

    pub(super) fn simulate_card_staff(&mut self, _delta: Duration) {}
}
