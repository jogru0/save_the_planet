use std::time::Instant;

use crate::grid::{Cell, Grid};

use self::{cards::Cards, duration::Duration, quantity::Quantity};

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

mod quantity;

mod cards;

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

        self.simulate(delta);

        self.render(input, delta)
    }

    fn simulate(&mut self, delta: Duration) {
        self.simulate_cards(delta);
    }
}

pub mod render;
