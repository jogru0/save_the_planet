use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

use super::duration::GRANULARITY;

pub mod balance;

impl Resource {
    fn increase_residual(&mut self, increase: u128) {
        self.residual += increase;
        let inc = self.residual / GRANULARITY;
        self.residual %= GRANULARITY;

        // assert!(inc == 0 || inc == 1);
        self.amount += inc;
    }

    pub fn fraction(n: u128, d: u128) -> Self {
        assert_ne!(d, 0);
        assert_eq!(GRANULARITY % d, 0);
        let full = n / d;
        let rest = n % d;

        let residual = (GRANULARITY / d) * rest;

        Self {
            amount: full,
            residual,
        }
    }
}

impl AddAssign for Resource {
    fn add_assign(&mut self, rhs: Self) {
        self.amount += rhs.amount;
        self.increase_residual(rhs.residual);
    }
}

impl Mul<u128> for Resource {
    type Output = Resource;

    fn mul(mut self, rhs: u128) -> Self::Output {
        let multiples_of_granularity = rhs / GRANULARITY;
        let rest = rhs % GRANULARITY;

        let old_residual = self.residual;
        self.residual = 0;

        self.amount *= rhs;
        self.amount += old_residual * multiples_of_granularity;

        self.increase_residual(old_residual * rest);

        self
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.amount as f64 + (self.residual as f64) / GRANULARITY as f64
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Resource {
    amount: u128,
    residual: u128,
}

impl Add for Resource {
    type Output = Resource;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Resource {
    pub fn as_f64(&self) -> f64 {
        self.amount as f64 + (self.residual as f64) / GRANULARITY as f64
    }

    pub fn whole_amount(&self) -> u128 {
        self.amount
    }

    pub fn divide_exactly(&mut self, divisor: u128) {
        assert_ne!(divisor, 0);

        let amount_not_accounted_for = self.amount % divisor;
        self.amount /= divisor;
        self.residual += amount_not_accounted_for * GRANULARITY;

        let residual_not_accounted_for = self.residual % divisor;
        self.residual /= divisor;

        assert_eq!(residual_not_accounted_for, 0);
        assert!(self.residual < GRANULARITY)
    }

    pub fn new(amount: u128) -> Self {
        Self {
            amount,
            residual: 0,
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
    }
}
