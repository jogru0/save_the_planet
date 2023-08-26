use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::duration::GRANULARITY;

pub mod balance;

impl<Q: QuantityType> Quantity<Q> {
    fn increase_residual(&mut self, increase: u128) {
        self.residual += increase;
        let inc = self.residual / GRANULARITY;
        self.residual %= GRANULARITY;

        // assert!(inc == 0 || inc == 1);
        self.amount += inc;
    }

    pub const fn fraction(n: u128, d: u128) -> Self {
        assert!(d != 0);
        assert!(GRANULARITY % d == 0);
        let full = n / d;
        let rest = n % d;

        let residual = (GRANULARITY / d) * rest;

        Self {
            amount: full,
            residual,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn saturating_sub(&mut self, amount: Self) -> Self {
        if &amount <= self {
            *self -= amount;
            amount
        } else {
            let actual = *self;
            *self = Self::default();
            actual
        }
    }
}

impl<Q: QuantityType> AddAssign for Quantity<Q> {
    fn add_assign(&mut self, rhs: Self) {
        self.amount += rhs.amount;
        self.increase_residual(rhs.residual);
    }
}

impl<Q: QuantityType> Mul<u128> for Quantity<Q> {
    type Output = Self;

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

pub trait QuantityType: Default + Debug + Copy + PartialEq {
    fn strinigy(amount: u128, resdiual: u128, accuracy: usize) -> String;
}

pub mod types;

pub struct SignedQuantity<Q: QuantityType> {
    absolute_value: Quantity<Q>,
    is_not_negative: bool,
}

impl<Q: QuantityType> Mul<Quantity<Q>> for u128 {
    type Output = Quantity<Q>;

    fn mul(self, mut rhs: Quantity<Q>) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl<Q: QuantityType> MulAssign<u128> for Quantity<Q> {
    fn mul_assign(&mut self, rhs: u128) {
        let small_part = rhs % GRANULARITY;
        let big_part = rhs / GRANULARITY;

        self.amount *= rhs;

        let residual = self.residual;
        self.residual = 0;

        self.amount += big_part * residual;
        self.increase_residual(small_part * residual);
    }
}

impl<Q: QuantityType> SignedQuantity<Q> {
    pub fn stringify(&self, prec: usize) -> String {
        let sign_char = if self.is_not_negative { '+' } else { '-' };
        format!("{}{}", sign_char, self.absolute_value.stringify(prec))
    }
}

impl<Q: QuantityType> PartialEq<Quantity<Q>> for SignedQuantity<Q> {
    fn eq(&self, other: &Quantity<Q>) -> bool {
        self.is_not_negative && &self.absolute_value == other
    }
}

impl<Q: QuantityType> PartialOrd<Quantity<Q>> for SignedQuantity<Q> {
    fn partial_cmp(&self, other: &Quantity<Q>) -> Option<std::cmp::Ordering> {
        if self.is_not_negative {
            self.absolute_value.partial_cmp(other)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Quantity<Q: QuantityType> {
    amount: u128,
    residual: u128,
    _phantom: PhantomData<Q>,
}

impl<Q: QuantityType> Add for Quantity<Q> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<Q: QuantityType> Sub for Quantity<Q> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<Q: QuantityType> PartialOrd for Quantity<Q> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn to_tuple<P: QuantityType>(s: &Quantity<P>) -> (u128, u128) {
            (s.amount, s.residual)
        }

        to_tuple(self).partial_cmp(&to_tuple(other))
    }
}

impl<Q: QuantityType> SubAssign for Quantity<Q> {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(rhs <= *self);
        let (spillover, res) = if rhs.residual <= self.residual {
            (0, self.residual - rhs.residual)
        } else {
            (1, GRANULARITY + self.residual - rhs.residual)
        };

        self.amount -= rhs.amount + spillover;
        self.residual = res;
    }
}

impl<Q: QuantityType> Quantity<Q> {
    pub fn stringify(&self, accuracy: usize) -> String {
        Q::strinigy(self.amount, self.residual, accuracy)
    }

    pub fn whole_amount(&self) -> u128 {
        self.amount
    }

    #[must_use]
    pub const fn divide_exactly(mut self, divisor: u128) -> Self {
        assert!(divisor != 0);

        let amount_not_accounted_for = self.amount % divisor;
        self.amount /= divisor;
        self.residual += amount_not_accounted_for * GRANULARITY;

        let residual_not_accounted_for = self.residual % divisor;
        self.residual /= divisor;

        assert!(residual_not_accounted_for == 0);
        assert!(self.residual < GRANULARITY);

        self
    }

    pub const fn new(amount: u128) -> Self {
        Self {
            amount,
            residual: 0,
            _phantom: PhantomData,
        }
    }

    pub fn try_pay(&mut self, val: Self) -> bool {
        if &val <= self {
            *self -= val;
            true
        } else {
            false
        }
    }
}

impl<Q: QuantityType> AddAssign<u128> for Quantity<Q> {
    fn add_assign(&mut self, rhs: u128) {
        self.amount += rhs;
    }
}
