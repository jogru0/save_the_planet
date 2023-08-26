use crate::{
    grid::{Cell, Grid, MutGridView},
    world::{rate::Rate, Card, Event},
};

use super::{duration::Duration, quantity::Quantity, Input, Key, Milestones, World};

pub const LINES_MAIN_FRAME_CONTENT: usize = 10;
pub const LINES_MAIN_FRAME: usize = LINES_MAIN_FRAME_CONTENT + 2;
pub const LINES_MESSAGES: usize = 3;
pub const LINES_GRID: usize = LINES_MAIN_FRAME + LINES_MESSAGES;

pub const CHARS_MENU: usize = 13;
pub const CHARS_CARD: usize = 23;
pub const CHARS_GRID: usize = CHARS_MENU + CHARS_CARD + 3;

impl World {
    fn render_messages(&self, input: &Input, delta: Duration, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MESSAGES);
        assert_eq!(view.width(), CHARS_GRID);

        view.print(
            0,
            0,
            &format!("Mouse tile position: {}, {}", input.mouse_x, input.mouse_y),
        );
        view.print(1, 0, &format!("Delta: {}ms", delta.as_millis()));
        view.print(1, 16, &format!("FPS: {}", input.fps));

        if let Some(Event::Key(key)) = input.event {
            view.print(2, 0, &format!("Key code: {:?}", key));
        } else {
            view.print(2, 0, "Messages can appear here.")
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

    fn render_co2_card(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        let co2_card = &mut self.cards.co2;

        view.print(
            0,
            0,
            &format!(
                "Saved CO2e: {}",
                co2_card.emission_balance.balance().stringify(2)
            ),
        );

        match input.event {
            Some(Event::Key(Key::F)) => co2_card.flyer += 1,
            Some(Event::Key(Key::H)) => {
                if Quantity::fraction(1, 10) <= co2_card.unsupporting_people
                    && co2_card.flyer.try_pay(Quantity::new(1))
                {
                    let previous_supporting_people = co2_card.supporting_people.whole_amount();
                    co2_card.supporting_people += Quantity::fraction(1, 10);
                    co2_card.unsupporting_people -= Quantity::fraction(1, 10);
                    let new_supporters =
                        co2_card.supporting_people.whole_amount() - previous_supporting_people;
                    assert!(new_supporters == 0 || new_supporters == 1);

                    co2_card.save_rate_from_flyers +=
                        Rate::new(Quantity::new(100_000) * new_supporters, Duration::YEAR)
                }
            }
            _ => {}
        }

        view.print(1, 0, &format!("Flyer: {}", co2_card.flyer.stringify(0)));
        view.print(
            2,
            0,
            &format!(" Rate: {}", co2_card.save_rate_from_flyers.stringify(4)),
        );
        view.print(
            3,
            0,
            &format!(
                " Supp: {} / {}",
                co2_card.supporting_people.stringify(0),
                &(co2_card.supporting_people + co2_card.unsupporting_people).stringify(0)
            ),
        );
    }

    pub fn render(&mut self, input: &Input, delta: Duration) -> Grid<Cell> {
        let mut grid = Grid::new(LINES_GRID, CHARS_GRID, Cell::new());
        let mut view = grid.view();

        if self.cards.milestones.is_none()
            && self.cards.co2.emission_balance.balance() >= (Quantity::new(1000))
        {
            self.cards.milestones = Some(Milestones::new())
        }

        let mut top_view = view.sub_view(0, 0, LINES_MAIN_FRAME, CHARS_GRID);
        top_view.fill_foreground(self.cards.get_card(self.cards.selected).color());

        if 2 <= self.cards.available_cards().len() {
            self.render_main_navigation(input, top_view);
        } else {
            self.render_main_card(input, top_view)
        }

        let bottom_view: MutGridView<'_, Cell> =
            view.sub_view(LINES_MAIN_FRAME, 0, LINES_MESSAGES, CHARS_GRID);
        self.render_messages(input, delta, bottom_view);

        grid
    }

    fn render_main_card(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME);
        assert_eq!(view.width(), CHARS_GRID);

        let inner = view.block();
        self.render_co2_card(input, inner);
    }

    fn render_card(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME_CONTENT);
        assert!(CHARS_CARD <= view.width());

        match self.cards.selected {
            Card::CO2 => self.render_co2_card(input, view),
            Card::Milestones => self.render_milestones(input, view),
        }
    }

    fn render_milestones(&self, _input: &Input, mut view: MutGridView<'_, Cell>) {
        view.print_overflowing(0, "Here will be milestones. Trust me.")
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
            view.print(line, char_id, &self.cards.get_card(card).menu_string());
        }
    }
}
