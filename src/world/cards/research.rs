use crate::grid::Color;

use super::AbstractCard;

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
