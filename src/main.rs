use std::error::Error;

use world::World;

mod grid;

mod bterm;
mod tui;

mod world;

mod input {

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

    pub struct Input {
        pub event: Option<Event>,
        pub mouse_x: usize,
        pub mouse_y: usize,
        pub fps: f32,
    }
}

mod duration {

    //TODO
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

    pub const NANOSECONDS_PER_TICK: u128 = 10;
    pub const TICKS_PER_MICROSECOND: u128 = NANO / MICRO / NANOSECONDS_PER_TICK;

    pub const TICKS_PER_MINUTE: u128 = TICKS_PER_SECOND * SECONDS_PER_MIN;
    pub const TICKS_PER_HOUR: u128 = TICKS_PER_MINUTE * MINS_PER_HOUR;
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
    pub const NANO: u128 = 1_000 * MICRO;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
    pub struct Duration {
        ticks: u128,
    }

    impl SubAssign for Duration {
        fn sub_assign(&mut self, rhs: Self) {
            self.ticks -= rhs.ticks
        }
    }

    impl AddAssign for Duration {
        fn add_assign(&mut self, rhs: Self) {
            self.ticks += rhs.ticks
        }
    }

    impl Sub for Duration {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl Add for Duration {
        type Output = Self;

        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
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
        pub const fn ticks(&self) -> u128 {
            self.ticks
        }

        pub const SECOND: Self = Duration {
            ticks: TICKS_PER_SECOND,
        };
        pub const MINUTE: Self = Duration {
            ticks: TICKS_PER_MINUTE,
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

        pub(crate) fn from_time_duration_rounded_down(time_duration: std::time::Duration) -> Self {
            Self {
                ticks: time_duration.as_nanos() / NANOSECONDS_PER_TICK,
            }
        }
    }
}

mod reality {
    use std::time::Instant;

    use crate::{
        duration::Duration,
        grid::{Cell, Grid},
        input::Input,
        world::World,
    };

    pub struct Reality {
        simulation: World,
        simulation_start_time: Option<Instant>,
        ticks_at_simulation_start: Duration,
    }
    impl Reality {
        pub fn new(world: World) -> Self {
            Self {
                ticks_at_simulation_start: world.get_total_ticks(),
                simulation: world,
                simulation_start_time: None,
            }
        }

        pub fn update(&mut self, input: &Input) -> Grid<Cell> {
            let current_time = Instant::now();
            let ticks_since_simulation_start =
                if let Some(simulation_start_time) = self.simulation_start_time {
                    assert!(simulation_start_time <= current_time);
                    Duration::from_time_duration_rounded_down(current_time - simulation_start_time)
                } else {
                    self.simulation_start_time = Some(current_time);
                    Duration::INSTANT
                };

            let total_ticks = ticks_since_simulation_start + self.ticks_at_simulation_start;

            self.simulation.simulate(total_ticks);
            self.simulation.render(input)
        }
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let world = World::new();

    let use_terminal = std::env::args().nth(1) == Some("--terminal".into());

    if use_terminal {
        tui::main(world)
    } else {
        bterm::main(world)
    }
}
