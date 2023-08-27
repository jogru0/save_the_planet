use crate::{
    duration::Duration,
    grid::{Cell, Color, MutGridView},
    input::Input,
    world::{
        message::Message,
        quantity::{
            balance::Balance,
            types::{Emission, Flyer, Person},
            Quantity,
        },
        rate::Rate,
        World,
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

pub enum Stage {
    Prolog { step: usize },
    Main,
}
impl Stage {
    fn step_forward(&mut self) {
        match self {
            Stage::Prolog { step } => {
                *step += 1;
                if step == &NUMBER_OF_PROLOG_STEPS {
                    *self = Stage::Main
                }
            }
            Stage::Main => unreachable!(),
        }
    }
}

mod main {
    use crate::{
        grid::{Cell, MutGridView},
        input::{Event, Input, Key},
        world::World,
    };

    impl World {
        pub(super) fn render_activism_main(
            &mut self,
            input: &Input,
            mut view: MutGridView<'_, Cell>,
        ) {
            match input.event {
                Some(Event::Key(Key::F)) => {
                    self.print_flyer();
                }
                Some(Event::Key(Key::H)) => {
                    self.handout_flyer();
                }
                _ => {}
            }

            let activism = &mut self.cards.activism;

            view.print(
                0,
                0,
                format!(
                    "Saved CO2e: {}",
                    activism.emission_balance.balance().stringify(2)
                )
                .into(),
            );

            view.print(
                1,
                0,
                format!("Flyer: {}", activism.flyer.stringify(0)).into(),
            );
            view.print(
                2,
                0,
                format!(" Rate: {}", activism.save_rate_from_flyers.stringify(4)).into(),
            );
            view.print(
                3,
                0,
                format!(
                    " Supp: {} / {}",
                    activism.supporting_people.stringify(0),
                    &(activism.supporting_people + activism.unsupporting_people).stringify(0)
                )
                .into(),
            );
        }
    }
}
mod prolog {
    use crate::{
        grid::{text::Text, Cell, MutGridView},
        input::{Event, Input, Key},
        world::{quantity::Quantity, World},
    };

    use super::{INITIAL_NUMBER_OF_FLYERS_AND_INVERSE_OF_PERSUASIVENESS, NUMBER_OF_PROLOG_STEPS};

    const FLYER_HANDOUT_TEXTS: [&str;
        INITIAL_NUMBER_OF_FLYERS_AND_INVERSE_OF_PERSUASIVENESS as usize] =
        ["10", "9", "8", "7", "6", "5", "4", "3", "2", "Last Hope"];

    const FLYER_PRINT_TEXTS: [&str; NUMBER_OF_PROLOG_STEPS] = [
        "need to print more",
        "oh no, not enough saved",
        "still need to wait xyz minutes",
        "dont be impatient",
        "on the other hand",
        "maybe its fine to",
    ];

    impl World {
        pub(super) fn render_activism_prolog(
            &mut self,
            input: &Input,
            view: MutGridView<'_, Cell>,
        ) {
            if self.cards.activism.flyer != Quantity::default() {
                self.render_flyer_handout(input, view);
            } else {
                self.render_flyer_print(input, view);
            }
        }

        fn render_flyer_handout(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
            let text_id = (INITIAL_NUMBER_OF_FLYERS_AND_INVERSE_OF_PERSUASIVENESS
                - self.cards.activism.flyer.whole_amount()) as usize;
            let text = FLYER_HANDOUT_TEXTS[text_id];

            view.print_overflowing(0, &Text::new().raw(text));

            if let Some(Event::Key(Key::H)) = input.event {
                let success = self.handout_flyer();
                assert!(success);
            }
        }

        fn render_flyer_print(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
            let step = match self.cards.activism.stage {
                super::Stage::Prolog { step } => step,
                super::Stage::Main => unreachable!(),
            };
            let text = FLYER_PRINT_TEXTS[step];

            view.print_overflowing(0, &Text::new().raw(text));

            if let Some(Event::Key(Key::F)) = input.event {
                let success = self.print_flyer();
                assert_eq!(success, step == NUMBER_OF_PROLOG_STEPS - 1);
                self.cards.activism.stage.step_forward();
                if step == NUMBER_OF_PROLOG_STEPS - 2 {
                    self.set_maximal_emission_deficit(Quantity::new(1000));
                }
            }
        }
    }
}

const INITIAL_NUMBER_OF_FLYERS_AND_INVERSE_OF_PERSUASIVENESS: u128 = 10;
const INITIAL_FLYER_PRINT_COST: Quantity<Emission> = Quantity::new(6);
const INITIAL_FLYER_PERSUASIVENESS: Quantity<Person> =
    Quantity::fraction(1, INITIAL_NUMBER_OF_FLYERS_AND_INVERSE_OF_PERSUASIVENESS);
const INITIAL_FLYER_EFFECTIVENESS: Rate<Emission> =
    Rate::new(Quantity::new(100_000), Duration::YEAR);
const NUMBER_OF_PROLOG_STEPS: usize = 6;

// #[derive(Debug)]
pub struct Activism {
    stage: Stage,
    pub emission_balance: Balance<Emission>,
    pub flyer: Quantity<Flyer>,
    pub supporting_people: Quantity<Person>,
    pub unsupporting_people: Quantity<Person>,
    pub save_rate_from_flyers: Rate<Emission>,

    pub flyer_persuasiveness: Quantity<Person>,
    pub flyer_effectiveness: Rate<Emission>,
    pub flyer_print_cost: Quantity<Emission>,
    pub maximal_emission_deficit: Quantity<Emission>,
}
impl Activism {
    pub fn new() -> Activism {
        Self {
            stage: Stage::Prolog { step: 0 },

            emission_balance: Balance::new(),
            flyer: Quantity::new(INITIAL_NUMBER_OF_FLYERS_AND_INVERSE_OF_PERSUASIVENESS),
            supporting_people: Quantity::new(0),
            unsupporting_people: Quantity::new(9_000_000_000),
            save_rate_from_flyers: Rate::new(Quantity::new(0), Duration::TICK),

            flyer_persuasiveness: INITIAL_FLYER_PERSUASIVENESS,
            flyer_effectiveness: INITIAL_FLYER_EFFECTIVENESS,
            flyer_print_cost: INITIAL_FLYER_PRINT_COST,
            maximal_emission_deficit: Quantity::default(),
        }
    }
}

impl World {
    fn set_maximal_emission_deficit(&mut self, new_maximal_deficit: Quantity<Emission>) {
        assert!(self.cards.activism.maximal_emission_deficit < new_maximal_deficit);
        self.cards.activism.maximal_emission_deficit = new_maximal_deficit;

        self.messages.queue(Message::new(
            format!(
                "Increased maximal emission deficit to {}",
                new_maximal_deficit.stringify(2)
            ),
            10 * Duration::SECOND,
        ))
    }

    pub(super) fn render_card_activism(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        match self.cards.activism.stage {
            Stage::Prolog { .. } => self.render_activism_prolog(input, view),
            Stage::Main => self.render_activism_main(input, view),
        }
    }

    pub(super) fn simulate_card_activism(&mut self, delta: Duration) {
        let co2_card = &mut self.cards.activism;

        *co2_card.emission_balance.pos_mut() += co2_card.save_rate_from_flyers * delta
    }

    fn handout_flyer(&mut self) -> bool {
        let activism = &mut self.cards.activism;

        if activism.unsupporting_people > Quantity::default()
            && activism.flyer.try_pay(Quantity::new(1))
        {
            let previous_supporting_people = activism.supporting_people.whole_amount();
            let reduction = activism
                .unsupporting_people
                .saturating_sub(activism.flyer_persuasiveness);
            activism.supporting_people += reduction;
            let new_supporters =
                activism.supporting_people.whole_amount() - previous_supporting_people;
            assert!(new_supporters == 0 || new_supporters == 1);

            activism.save_rate_from_flyers += new_supporters * activism.flyer_effectiveness;

            true
        } else {
            false
        }
    }

    fn print_flyer(&mut self) -> bool {
        let mut theoretical_balance = self.cards.activism.emission_balance;
        *theoretical_balance.neg_mut() += self.cards.activism.flyer_print_cost;
        *theoretical_balance.pos_mut() += self.cards.activism.maximal_emission_deficit;

        if theoretical_balance.balance() < Quantity::default() {
            return false;
        }

        *self.cards.activism.emission_balance.neg_mut() += self.cards.activism.flyer_print_cost;
        self.cards.activism.flyer += 1;
        true
    }
}
