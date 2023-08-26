use crate::{
    grid::Color,
    world::quantity::{types::Person, Quantity},
};

use super::AbstractCard;

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
    researcher: Quantity<Person>,
    activists: Quantity<Person>,
}
impl Staff {
    pub fn new() -> Staff {
        Staff {
            researcher: Quantity::default(),
            activists: Quantity::default(),
        }
    }
}
