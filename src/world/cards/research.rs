use crate::{
    duration::Duration,
    grid::{Cell, Color, MutGridView},
    input::Input,
    world::{
        quantity::{
            types::{Emission, Person, ResearchPoints},
            Quantity,
        },
        rate::Rate,
        World,
    },
};

use super::abstract_card::AbstractCard;

#[derive(Debug, Clone, Copy)]
enum Project {
    BetterGuidelines1,
    BetterGuidelines2,
    CatchierFlyer1,
}

const FLYER_EFFECTIVENESS_0: Rate<Emission> = Rate::new(Quantity::new(100_000), Duration::YEAR);
const FLYER_EFFECTIVENESS_1: Rate<Emission> = Rate::new(Quantity::new(150_000), Duration::YEAR);
const FLYER_EFFECTIVENESS_2: Rate<Emission> = Rate::new(Quantity::new(500_000), Duration::YEAR);

const CATCHIER_FLYER_0: Quantity<Person> = Quantity::fraction(1, 10);
const CATCHIER_FLYER_1: Quantity<Person> = Quantity::fraction(1, 5);

impl Project {
    fn apply(self, world: &mut World) {
        match self {
            Project::BetterGuidelines1 => {
                let effectiveness = &mut world.cards.activism.flyer_effectiveness;
                assert_eq!(effectiveness, &FLYER_EFFECTIVENESS_0);
                *effectiveness = FLYER_EFFECTIVENESS_1;
            }
            Project::BetterGuidelines2 => {
                let effectiveness = &mut world.cards.activism.flyer_effectiveness;
                assert_eq!(effectiveness, &FLYER_EFFECTIVENESS_1);
                *effectiveness = FLYER_EFFECTIVENESS_2;
            }
            Project::CatchierFlyer1 => {
                let persuasiveness = &mut world.cards.activism.flyer_persuasiveness;
                assert_eq!(persuasiveness, &CATCHIER_FLYER_0);
                *persuasiveness = CATCHIER_FLYER_1;
            }
        }
    }

    fn cost(&self) -> Quantity<ResearchPoints> {
        match self {
            Project::BetterGuidelines1 => Quantity::new(1),
            Project::BetterGuidelines2 => Quantity::new(2),
            Project::CatchierFlyer1 => Quantity::new(5),
        }
    }
}

#[derive(Debug)]
pub struct Research {
    discovered: bool,
    active: Option<(Project, Quantity<ResearchPoints>)>,
}
impl Research {
    pub fn new() -> Research {
        Research {
            active: None,
            discovered: false,
        }
    }
}

impl AbstractCard for Research {
    fn menu_string(&self) -> String {
        "Research".into()
    }

    fn color(&self) -> Color {
        Color::RED
    }

    fn is_visible(&self) -> bool {
        self.discovered
    }
}

impl World {
    pub(super) fn render_card_research(&mut self, _input: &Input, mut view: MutGridView<'_, Cell>) {
        if let Some((res, progress)) = self.cards.research.active {
            view.print(
                0,
                0,
                format!(
                    "Progress: {} / {}",
                    progress.stringify(2),
                    res.cost().stringify(2)
                )
                .into(),
            );
        }
    }

    pub(super) fn simulate_card_research(&mut self, delta: Duration) {
        if !self.cards.research.discovered
            && self.cards.activism.supporting_people >= Quantity::new(4)
        {
            self.cards.research.discovered = true;
            self.cards.research.active = Some((Project::BetterGuidelines1, Quantity::default()))
        }

        if let Some((res, progress)) = &mut self.cards.research.active {
            *progress += Rate::new(Quantity::new(1), Duration::MINUTE) * delta;
            if progress >= &mut res.cost() {
                res.apply(self);
                self.cards.research.active = None;
            }
        }
    }
}
