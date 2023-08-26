use crate::{
    grid::{Cell, Color, MutGridView},
    world::{duration::Duration, Input, World},
};

use super::abstract_card::AbstractCard;

#[derive(Debug)]
pub struct Research {}
impl Research {
    pub fn new() -> Research {
        Research {}
    }
}

impl AbstractCard for Research {
    fn menu_string(&self) -> String {
        "Research".into()
    }

    fn color(&self) -> Color {
        Color::RED
    }

    fn is_visible(&self) -> bool {
        false
    }
}

impl World {
    pub(super) fn render_card_research(
        &mut self,
        _input: &Input,
        mut _view: MutGridView<'_, Cell>,
    ) {
    }

    pub(super) fn simulate_card_research(&mut self, _delta: Duration) {}
}
