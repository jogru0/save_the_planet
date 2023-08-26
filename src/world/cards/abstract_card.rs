use crate::grid::Color;

pub trait AbstractCard {
    fn menu_string(&self) -> String;
    fn color(&self) -> Color;
    fn is_visible(&self) -> bool;
}
