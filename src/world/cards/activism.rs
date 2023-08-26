use crate::{
    grid::{Cell, Color, MutGridView},
    world::{
        duration::Duration,
        quantity::{
            balance::Balance,
            types::{Emission, Flyer, Person},
            Quantity,
        },
        rate::Rate,
        Event, Input, Key, World,
    },
};

use super::abstract_card::AbstractCard;

impl AbstractCard for Activism {
    fn menu_string(&self) -> String {
        "CO2".into()
    }

    fn color(&self) -> Color {
        Color::CYAN
    }

    fn is_visible(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct Activism {
    pub emission_balance: Balance<Emission>,
    pub flyer: Quantity<Flyer>,
    pub supporting_people: Quantity<Person>,
    pub unsupporting_people: Quantity<Person>,
    pub save_rate_from_flyers: Rate<Emission>,
}
impl Activism {
    pub fn new() -> Activism {
        Self {
            emission_balance: Balance::new(),
            flyer: Quantity::new(10),
            supporting_people: Quantity::new(0),
            unsupporting_people: Quantity::new(9_000_000_000),
            save_rate_from_flyers: Rate::new(Quantity::new(0), Duration::TICK),
        }
    }
}

impl World {
    pub fn render_co2_card(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        let co2_card = &mut self.cards.co2;

        view.print(
            0,
            0,
            &format!(
                "Saved CO2e: {}",
                co2_card.emission_balance.balance().stringify(2)
            ),
        );

        match input.event {
            Some(Event::Key(Key::F)) => co2_card.flyer += 1,
            Some(Event::Key(Key::H)) => {
                if Quantity::fraction(1, 10) <= co2_card.unsupporting_people
                    && co2_card.flyer.try_pay(Quantity::new(1))
                {
                    let previous_supporting_people = co2_card.supporting_people.whole_amount();
                    co2_card.supporting_people += Quantity::fraction(1, 10);
                    co2_card.unsupporting_people -= Quantity::fraction(1, 10);
                    let new_supporters =
                        co2_card.supporting_people.whole_amount() - previous_supporting_people;
                    assert!(new_supporters == 0 || new_supporters == 1);

                    co2_card.save_rate_from_flyers +=
                        Rate::new(Quantity::new(100_000) * new_supporters, Duration::YEAR)
                }
            }
            _ => {}
        }

        view.print(1, 0, &format!("Flyer: {}", co2_card.flyer.stringify(0)));
        view.print(
            2,
            0,
            &format!(" Rate: {}", co2_card.save_rate_from_flyers.stringify(4)),
        );
        view.print(
            3,
            0,
            &format!(
                " Supp: {} / {}",
                co2_card.supporting_people.stringify(0),
                &(co2_card.supporting_people + co2_card.unsupporting_people).stringify(0)
            ),
        );
    }

    pub fn simulate_card_co2(&mut self, delta: Duration) {
        let co2_card = &mut self.cards.co2;

        *co2_card.emission_balance.pos_mut() += co2_card.save_rate_from_flyers * delta
    }
}
