use std::time::Instant;

use crate::grid::{Cell, Color, Grid, MutGridView};

use self::{
    duration::Duration,
    rate::Rate,
    resource::{balance::Balance, Resource},
};

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

mod rate;

mod resource;

#[derive(Debug)]
struct CO2 {
    emission_balance: Balance,
    flyer: Resource,
    supporting_people: Resource,
    unsupporting_people: Resource,
    save_rate_from_flyers: Rate,
}
impl CO2 {
    fn new() -> CO2 {
        Self {
            emission_balance: Balance::new(),
            flyer: Resource::new(10),
            supporting_people: Resource::new(0),
            unsupporting_people: Resource::new(9_000_000_000),
            save_rate_from_flyers: Rate::new(Resource::new(0), Duration::TICK),
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
    CO2,
    Milestones,
}

impl AbstractCard for CO2 {
    fn menu_string(&self) -> String {
        "CO2".into()
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
    co2: CO2,
    milestones: Option<Milestones>,
}
impl Cards {
    fn new() -> Cards {
        Self {
            selected: Card::CO2,
            co2: CO2::new(),
            milestones: None,
        }
    }

    fn available_cards(&self) -> Vec<Card> {
        let mut result = vec![Card::CO2];

        if self.milestones.is_some() {
            result.push(Card::Milestones);
        }

        result
    }

    fn get_card(&self, card: Card) -> &dyn AbstractCard {
        match card {
            Card::CO2 => &self.co2,
            Card::Milestones => self.milestones.as_ref().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct World {
    cards: Cards,
    current_time: Option<Instant>,
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

mod duration {
    use std::ops::{Mul, MulAssign};

    pub const TICKS_PER_MICROSECOND: u128 = 100;

    pub const TICKS_PER_MIN: u128 = TICKS_PER_SECOND * SECONDS_PER_MIN;
    pub const TICKS_PER_HOUR: u128 = TICKS_PER_MIN * MINS_PER_HOUR;
    pub const TICKS_PER_DAY: u128 = TICKS_PER_HOUR * HOURS_PER_DAY;

    pub const TICKS_PER_YEAR: u128 = TICKS_PER_DAY * DAYS_PER_YEAR;

    pub const TICKS_PER_SECOND: u128 = TICKS_PER_MICROSECOND * MICRO;
    pub const TICKS_PER_MILLISECOND: u128 = TICKS_PER_MICROSECOND * MICRO / MILLI;

    pub const SECONDS_PER_MIN: u128 = 60;
    pub const MINS_PER_HOUR: u128 = 60;
    pub const HOURS_PER_DAY: u128 = 24;

    // pub const DAYS_PER_WEEK: u128 = 7;
    // pub const DAYS_PER_MONTH: u128 = 30;
    pub const DAYS_PER_YEAR: u128 = 365;
    pub const DAYS_UNTIL_WEEK_MONTH_AND_YEAR_ALIGN: u128 = 15330;
    // pub const DAYS_UNTIL_WEEK_AND_YEAR_ALIGN: u128 = 2555;

    pub const FACTOR_FOR_EXACT_DECIMAL_NUMBERS_WITH_TWO_DIGITS: u128 = 100;

    pub const GRANULARITY: u128 = TICKS_PER_DAY
        * DAYS_UNTIL_WEEK_MONTH_AND_YEAR_ALIGN
        * FACTOR_FOR_EXACT_DECIMAL_NUMBERS_WITH_TWO_DIGITS;

    pub const _SQU: u128 = GRANULARITY * GRANULARITY;

    pub const MILLI: u128 = 1_000;
    pub const MICRO: u128 = 1_000 * MILLI;
    // pub const NANO: u128 = 1_000 * MICRO;

    #[derive(Clone, Copy)]
    pub struct Duration {
        ticks: u128,
    }

    impl Mul<Duration> for u128 {
        type Output = Duration;

        fn mul(self, mut rhs: Duration) -> Self::Output {
            rhs *= self;
            rhs
        }
    }

    impl MulAssign<u128> for Duration {
        fn mul_assign(&mut self, rhs: u128) {
            self.ticks *= rhs;
        }
    }

    impl Duration {
        pub fn ticks(&self) -> u128 {
            self.ticks
        }

        pub const SECOND: Self = Duration {
            ticks: TICKS_PER_SECOND,
        };

        pub const MICROSECOND: Self = Duration {
            ticks: TICKS_PER_MICROSECOND,
        };

        pub const TICK: Self = Duration { ticks: 1 };
        pub const INSTANT: Self = Duration { ticks: 0 };

        pub const YEAR: Self = Duration {
            ticks: TICKS_PER_YEAR,
        };

        pub fn as_millis(&self) -> f64 {
            self.ticks as f64 / TICKS_PER_MILLISECOND as f64
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            cards: Cards::new(),
            current_time: None,
        }
    }

    pub fn update(&mut self, input: &Input) -> Grid<Cell> {
        let current_time = Instant::now();
        let delta = if let Some(time) = self.current_time {
            let micros = (current_time - time).as_micros();
            assert!(0 < micros);
            micros * Duration::MICROSECOND
        } else {
            Duration::INSTANT
        };
        self.current_time = Some(current_time);

        let mut grid = Grid::new(LINES_GRID, CHARS_GRID, Cell::new());
        let mut view = grid.view();

        if self.cards.milestones.is_none() && self.cards.co2.emission_balance.is_at_least(1000) {
            self.cards.milestones = Some(Milestones::new())
        }

        self.apply_rates(delta);

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
                co2_card.emission_balance.print_as_weight()
            ),
        );

        match input.event {
            Some(Event::Key(Key::F)) => co2_card.flyer += 1,
            Some(Event::Key(Key::H)) => {
                if 1 <= co2_card.unsupporting_people.whole_amount() && co2_card.flyer.try_pay(1) {
                    let previous_supporting_people = co2_card.supporting_people.whole_amount();
                    co2_card.supporting_people += Resource::fraction(1, 10);
                    let new_supporters =
                        co2_card.supporting_people.whole_amount() - previous_supporting_people;
                    assert!(new_supporters <= 1);
                    let success = co2_card.unsupporting_people.try_pay(new_supporters);
                    assert!(success);
                    co2_card.save_rate_from_flyers +=
                        Rate::new(Resource::new(100_000) * new_supporters, Duration::YEAR)
                }
            }
            _ => {}
        }

        view.print(1, 0, &format!("Flyer: {}", co2_card.flyer.as_f64()));
        view.print(2, 0, &format!(" Rate: {}", co2_card.save_rate_from_flyers));
        view.print(
            3,
            0,
            &format!(" Supp: {} /", co2_card.supporting_people.as_f64()),
        );
        view.print(
            4,
            7,
            &format!(
                "{}",
                (co2_card.supporting_people + co2_card.unsupporting_people).as_f64()
            ),
        );
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

    fn apply_rates(&mut self, delta: Duration) {
        let co2_card = &mut self.cards.co2;

        *co2_card.emission_balance.pos_mut() += co2_card.save_rate_from_flyers * delta
    }
}
