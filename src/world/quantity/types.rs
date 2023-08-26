use std::fmt::Debug;

use crate::world::duration::GRANULARITY;

use super::{Quantity, QuantityType};

fn get_conservative_f64(amount: u128, residual: u128, accuracy: usize) -> f64 {
    let accuracy_factor = 10_u128.pow(accuracy as u32);

    let res: Quantity<Flyer> =
        accuracy_factor * (Quantity::new(amount) + Quantity::fraction(residual, GRANULARITY));

    res.whole_amount() as f64 / accuracy_factor as f64
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Emission;

pub trait Number: Default + Debug + Copy + PartialEq {}

impl<N: Number> QuantityType for N {
    fn strinigy(amount: u128, resdiual: u128, accuracy: usize) -> String {
        format!("{}", get_conservative_f64(amount, resdiual, accuracy))
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Flyer;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Person;

impl Number for Flyer {}
impl Number for Person {}

impl QuantityType for Emission {
    fn strinigy(amount: u128, residual: u128, accuracy: usize) -> String {
        let f64 = get_conservative_f64(amount, residual, accuracy);
        format!("{}g", f64)
    }
}
