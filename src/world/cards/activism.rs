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
    pub flyer_persuasiveness: Quantity<Person>,
    pub flyer_effectiveness: Rate<Emission>,
}
impl Activism {
    pub fn new() -> Activism {
        Self {
            emission_balance: Balance::new(),
            flyer: Quantity::new(10),
            supporting_people: Quantity::new(0),
            unsupporting_people: Quantity::new(9_000_000_000),
            save_rate_from_flyers: Rate::new(Quantity::new(0), Duration::TICK),

            flyer_persuasiveness: Quantity::fraction(1, 10),
            flyer_effectiveness: Rate::new(Quantity::new(100_000), Duration::YEAR),
        }
    }
}

impl World {
    pub(super) fn render_card_activism(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        let co2_card = &mut self.cards.activism;

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
                if co2_card.unsupporting_people > Quantity::default()
                    && co2_card.flyer.try_pay(Quantity::new(1))
                {
                    let previous_supporting_people = co2_card.supporting_people.whole_amount();
                    let reduction = co2_card
                        .unsupporting_people
                        .saturating_sub(co2_card.flyer_persuasiveness);
                    co2_card.supporting_people += reduction;
                    let new_supporters =
                        co2_card.supporting_people.whole_amount() - previous_supporting_people;
                    assert!(new_supporters == 0 || new_supporters == 1);

                    co2_card.save_rate_from_flyers += new_supporters * co2_card.flyer_effectiveness
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

    pub(super) fn simulate_card_activism(&mut self, delta: Duration) {
        let co2_card = &mut self.cards.activism;

        *co2_card.emission_balance.pos_mut() += co2_card.save_rate_from_flyers * delta
    }
}
