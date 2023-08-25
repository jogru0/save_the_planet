use std::ops::AddAssign;

use crate::grid::{Cell, Color, Grid, MutGridView};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    Down,
    Up,
    Left,
    Right,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
}

pub enum Event {
    Key(Key),
}

#[derive(Debug)]
struct Resource {
    amount: u128,
    is_discovered: bool,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            amount: 0,
            is_discovered: false,
        }
    }

    pub fn try_pay(&mut self, val: u128) -> bool {
        if val <= self.amount {
            self.amount -= val;
            true
        } else {
            false
        }
    }
}

impl AddAssign<u128> for Resource {
    fn add_assign(&mut self, rhs: u128) {
        self.amount += rhs;
        self.is_discovered = true;
    }
}

#[derive(Debug)]
struct Flyer {
    saved: u128,
    flyer: Resource,
}
impl Flyer {
    fn new() -> Flyer {
        Self {
            saved: 0,
            flyer: Resource::new(),
        }
    }
}

#[derive(Debug)]
struct Milestones;
impl Milestones {
    fn new() -> Milestones {
        Self {}
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Card {
    Flyer,
    Milestones,
}

impl AbstractCard for Flyer {
    fn menu_string(&self) -> String {
        "Flyer".into()
    }

    fn color(&self) -> Color {
        Color::CYAN
    }
}

impl AbstractCard for Milestones {
    fn menu_string(&self) -> String {
        "Milestones".into()
    }

    fn color(&self) -> Color {
        Color::GOLD
    }
}

trait AbstractCard {
    fn menu_string(&self) -> String;
    fn color(&self) -> Color;
}

#[derive(Debug)]
struct Cards {
    selected: Card,
    flyer: Flyer,
    milestones: Option<Milestones>,
}
impl Cards {
    fn new() -> Cards {
        Self {
            selected: Card::Flyer,
            flyer: Flyer::new(),
            milestones: None,
        }
    }

    fn available_cards(&self) -> Vec<Card> {
        let mut result = vec![Card::Flyer];

        if self.milestones.is_some() {
            result.push(Card::Milestones);
        }

        result
    }

    fn get_card(&self, card: Card) -> &dyn AbstractCard {
        match card {
            Card::Flyer => &self.flyer,
            Card::Milestones => self.milestones.as_ref().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct World {
    cards: Cards,
}

pub struct Input {
    pub event: Option<Event>,
    pub mouse_x: usize,
    pub mouse_y: usize,
    pub fps: f32,
}

pub const LINES_MAIN_FRAME_CONTENT: usize = 10;
pub const LINES_MAIN_FRAME: usize = LINES_MAIN_FRAME_CONTENT + 2;
pub const LINES_MESSAGES: usize = 3;
pub const LINES_GRID: usize = LINES_MAIN_FRAME + LINES_MESSAGES;

pub const CHARS_MENU: usize = 13;
pub const CHARS_CARD: usize = 23;
pub const CHARS_GRID: usize = CHARS_MENU + CHARS_CARD + 3;

impl World {
    pub fn new() -> Self {
        Self {
            cards: Cards::new(),
        }
    }

    pub fn update(&mut self, input: &Input) -> Grid<Cell> {
        let mut grid = Grid::new(LINES_GRID, CHARS_GRID, Cell::new());
        let mut view = grid.view();

        if self.cards.milestones.is_none() && self.cards.flyer.saved >= 1000 {
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
        self.render_messages(input, bottom_view);

        grid
    }

    fn render_messages(&self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MESSAGES);
        assert_eq!(view.width(), CHARS_GRID);

        view.print(
            0,
            0,
            &format!("Mouse tile position: {}, {}", input.mouse_x, input.mouse_y),
        );
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

    fn render_flyer(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        let card = &mut self.cards.flyer;

        view.print(0, 0, &format!("Saved CO2e: {}kg", card.saved));

        match input.event {
            Some(Event::Key(Key::F)) => card.flyer += 1,
            Some(Event::Key(Key::H)) => {
                if card.flyer.try_pay(1) {
                    card.saved += 100;
                }
            }
            _ => {}
        }

        let flyer_line = if card.flyer.is_discovered {
            format!("Flyer: {}", card.flyer.amount)
        } else {
            "Press `f` to produce a flyer.".into()
        };

        view.print(1, 0, &flyer_line);
    }

    fn render_main_card(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME);
        assert_eq!(view.width(), CHARS_GRID);

        let inner = view.block();
        self.render_flyer(input, inner);
    }

    fn render_card(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        assert_eq!(view.height(), LINES_MAIN_FRAME_CONTENT);
        assert!(CHARS_CARD <= view.width());

        match self.cards.selected {
            Card::Flyer => self.render_flyer(input, view),
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
