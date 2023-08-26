use super::Resource;

#[derive(Debug)]
pub struct Balance {
    pos: Resource,
    neg: Resource,
}
impl Balance {
    pub fn pos_mut(&mut self) -> &mut Resource {
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

    pub fn print_as_weight(&self) -> String {
        let f64 = self.pos.as_f64() - self.neg.as_f64();
        format!("{}g", f64)
    }

    pub fn new() -> Balance {
        Balance {
            pos: Resource::default(),
            neg: Resource::default(),
        }
    }

    pub fn is_at_least(&self, arg: u128) -> bool {
        let spillover = if self.pos.residual < self.neg.residual {
            1
        } else {
            0
        };

        self.neg.amount + spillover + arg <= self.pos.amount
    }
}
