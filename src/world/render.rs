use crate::{
    grid::{text::Text, Cell, Color, Grid, MutGridView},
    input::{Event, Input, Key},
};

use super::World;

pub const LINES_MAIN_FRAME_CONTENT: usize = 10;
pub const LINES_MAIN_FRAME: usize = LINES_MAIN_FRAME_CONTENT + 2;
pub const LINES_MESSAGES: usize = 3;
pub const LINES_GRID: usize = LINES_MAIN_FRAME + LINES_MESSAGES;

pub const CHARS_MENU: usize = 13;
pub const CHARS_CARD: usize = 23;
pub const CHARS_GRID: usize = CHARS_MENU + CHARS_CARD + 3;

impl World {
    fn render_bottom_area(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MESSAGES);
        assert_eq!(view.width(), CHARS_GRID);

        view.print(
            0,
            0,
            format!("Mouse tile position: {}, {}", input.mouse_x, input.mouse_y).into(),
        );

        if let Some(Event::Key(key)) = input.event {
            view.print(2, 0, format!("Key code: {:?}", key).into());
        } else {
            view.print(2, 0, "Messages can appear here.".to_owned().into())
        }

        if let Some(message) = self.messages.get_current() {
            let color = match (self.total_ticks.as_millis() as u128 / 50) % 3 {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => unreachable!(),
            };

            view.print_overflowing(0, Text::new().styled(message.text(), None, Some(color)))
        }
    }

    fn render_main_navigation(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME);
        assert_eq!(view.width(), CHARS_GRID);

        let (left, pre_right) = view.split_block(CHARS_MENU);
        assert_eq!(pre_right.3, CHARS_CARD);

        self.render_menu(input, left);

        let right = view.sub_view(pre_right.0, pre_right.1, pre_right.2, pre_right.3);
        self.render_card(input, right);
    }

    pub fn render(&mut self, input: &Input) -> Grid<Cell> {
        let mut grid = Grid::new(LINES_GRID, CHARS_GRID, Cell::new());
        let mut view = grid.view();

        let mut top_view = view.sub_view(0, 0, LINES_MAIN_FRAME, CHARS_GRID);
        top_view.fill_foreground(self.cards.get_card(self.cards.selected).color());

        if 2 <= self.cards.available_cards().len() {
            self.render_main_navigation(input, top_view);
        } else {
            self.render_main_card(input, top_view)
        }

        let bottom_view: MutGridView<'_, Cell> =
            view.sub_view(LINES_MAIN_FRAME, 0, LINES_MESSAGES, CHARS_GRID);
        self.render_bottom_area(input, bottom_view);

        grid
    }

    fn render_main_card(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME);
        assert_eq!(view.width(), CHARS_GRID);

        let inner = view.block();
        self.render_card(input, inner);
    }

    fn render_menu(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME_CONTENT);
        assert_eq!(view.width(), CHARS_MENU);

        let available_cards = self.cards.available_cards();
        let mut current_pos = available_cards
            .iter()
            .position(|c| c == &self.cards.selected)
            .unwrap();

        match input.event {
            Some(Event::Key(Key::Down)) => {
                current_pos = (available_cards.len() - 1).min(current_pos + 1)
            }
            Some(Event::Key(Key::Up)) => current_pos = current_pos.saturating_sub(1),
            _ => {}
        }
        self.cards.selected = available_cards[current_pos];

        for (line, card) in available_cards.into_iter().enumerate() {
            let char_id = if card == self.cards.selected { 1 } else { 0 };

            let card = self.cards.get_card(card);

            view.print(
                line,
                char_id,
                Text::new().styled(&card.menu_string(), Some(card.color()), None),
            );
        }
    }
}
