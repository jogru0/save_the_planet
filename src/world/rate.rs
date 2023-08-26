use std::{
    fmt::Display,
    ops::{AddAssign, Mul},
};

use crate::world::duration::Duration;

use super::Resource;

#[derive(Debug, Clone, Copy)]
pub struct Rate {
    difference_per_tick: Resource,
}

impl Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/s", self.per(Duration::SECOND))
    }
}

impl AddAssign for Rate {
    fn add_assign(&mut self, rhs: Self) {
        self.difference_per_tick += rhs.difference_per_tick;
    }
}

impl Rate {
    pub fn new(mut increase: Resource, duration: Duration) -> Self {
        increase.divide_exactly(duration.ticks());
        Self {
            difference_per_tick: increase,
        }
    }

    fn per(self, duration: Duration) -> Resource {
        self * duration
    }
}

impl Mul<Duration> for Rate {
    type Output = Resource;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.difference_per_tick * rhs.ticks()
    }
}
