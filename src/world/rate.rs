use std::ops::{AddAssign, Mul, MulAssign};

use crate::world::duration::Duration;

use super::{quantity::QuantityType, Quantity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rate<Q: QuantityType> {
    difference_per_tick: Quantity<Q>,
}

impl<Q: QuantityType> AddAssign for Rate<Q> {
    fn add_assign(&mut self, rhs: Self) {
        self.difference_per_tick += rhs.difference_per_tick;
    }
}

impl<Q: QuantityType> Rate<Q> {
    pub const fn new(mut increase: Quantity<Q>, duration: Duration) -> Self {
        increase = increase.divide_exactly(duration.ticks());
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

impl<Q: QuantityType> MulAssign<u128> for Rate<Q> {
    fn mul_assign(&mut self, rhs: u128) {
        self.difference_per_tick *= rhs
    }
}

impl<Q: QuantityType> Mul<Rate<Q>> for u128 {
    type Output = Rate<Q>;

    fn mul(self, mut rhs: Rate<Q>) -> Self::Output {
        rhs *= self;
        rhs
    }
}
