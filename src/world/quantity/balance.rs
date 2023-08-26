use super::{Quantity, QuantityType, SignedQuantity};

#[derive(Debug)]
pub struct Balance<Q: QuantityType> {
    pos: Quantity<Q>,
    neg: Quantity<Q>,
}

impl<Q: QuantityType> Balance<Q> {
    pub fn balance(&self) -> SignedQuantity<Q> {
        let (is_not_negative, smaller, bigger) = if self.neg <= self.pos {
            (true, self.neg, self.pos)
        } else {
            (false, self.pos, self.neg)
        };

        SignedQuantity {
            absolute_value: bigger - smaller,
            is_not_negative,
        }
    }

    pub fn pos_mut(&mut self) -> &mut Quantity<Q> {
        &mut self.pos
    }

    // pub fn neg_mut(&mut self) -> &mut Resource {
    //     &mut self.neg
    // }

    // pub fn pos(&self) -> &Resource {
    //     &self.pos
    // }

    // pub fn neg(&self) -> &Resource {
    //     &self.neg
    // }

    pub fn new() -> Self {
        Self {
            pos: Quantity::default(),
            neg: Quantity::default(),
        }
    }
}
