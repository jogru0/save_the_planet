use crate::{
    duration::Duration,
    grid::{Cell, Color, MutGridView},
    input::Input,
    world::World,
};

use super::abstract_card::AbstractCard;

#[derive(Debug)]
pub struct Milestones {
    is_visible: bool,
}

impl Milestones {
    pub fn new() -> Milestones {
        Self { is_visible: false }
    }

    pub fn discover(&mut self) {
        assert!(!self.is_visible);
        self.is_visible = true;
    }
}

impl AbstractCard for Milestones {
    fn menu_string(&self) -> String {
        "Milestones".into()
    }

    fn color(&self) -> Color {
        Color::YELLOW
    }

    fn is_visible(&self) -> bool {
        false
    }
}

impl World {
    pub(super) fn render_card_milestones(
        &mut self,
        _input: &Input,
        mut _view: MutGridView<'_, Cell>,
    ) {
    }

    pub(super) fn simulate_card_milestones(&mut self, _delta: Duration) {}
}
