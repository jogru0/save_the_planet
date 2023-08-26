use std::ops::{AddAssign, Mul};

use crate::world::duration::Duration;

use super::{quantity::QuantityType, Quantity};

#[derive(Debug, Clone, Copy)]
pub struct Rate<Q: QuantityType> {
    difference_per_tick: Quantity<Q>,
}

impl<Q: QuantityType> AddAssign for Rate<Q> {
    fn add_assign(&mut self, rhs: Self) {
        self.difference_per_tick += rhs.difference_per_tick;
    }
}

impl<Q: QuantityType> Rate<Q> {
    pub fn new(mut increase: Quantity<Q>, duration: Duration) -> Self {
        increase.divide_exactly(duration.ticks());
        Self {
            difference_per_tick: increase,
        }
    }

    fn per(self, duration: Duration) -> Quantity<Q> {
        self * duration
    }

    pub fn stringify(&self, accuracy: usize) -> String {
        format!("{}/s", self.per(Duration::SECOND).stringify(accuracy))
    }
}

impl<Q: QuantityType> Mul<Duration> for Rate<Q> {
    type Output = Quantity<Q>;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.difference_per_tick * rhs.ticks()
    }
}
