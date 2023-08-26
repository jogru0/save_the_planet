use crate::grid::Color;

use super::AbstractCard;

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
