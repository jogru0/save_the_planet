use indexmap::IndexSet;

use crate::{
    duration::Duration,
    grid::{text::Text, Cell, Color, MutGridView},
    input::{Event, Input, Key},
    world::{
        message::{Message, STANDARD_MESSAGE_DURATION},
        quantity::{
            types::{Emission, Person, ResearchPoints},
            Quantity,
        },
        rate::Rate,
        World,
    },
};

use self::research_manager::ResearchManager;

use super::abstract_card::AbstractCard;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Project {
    BetterGuidelines1,
    BetterGuidelines2,
    CatchierFlyer1,
}

const ALL_PROJECTS: [Project; 3] = [
    Project::BetterGuidelines1,
    Project::BetterGuidelines2,
    Project::CatchierFlyer1,
];

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

                world
                    .cards
                    .research
                    .manager
                    .unlock(Project::BetterGuidelines2);
                world.cards.research.manager.unlock(Project::CatchierFlyer1);
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

    fn name(&self) -> String {
        match self {
            Project::BetterGuidelines1 => "Better Guidelines".into(),
            Project::BetterGuidelines2 => "Even better Guidelines".into(),
            Project::CatchierFlyer1 => "Catchier Flyer".into(),
        }
    }
}

mod research_manager {
    use indexmap::IndexSet;

    use crate::{
        duration::Duration,
        world::{
            quantity::{types::ResearchPoints, Quantity},
            World,
        },
    };

    use super::{Project, ALL_PROJECTS};

    pub struct ResearchManager {
        locked: IndexSet<Project>,
        active: Option<(Project, Quantity<ResearchPoints>)>,
        available: IndexSet<Project>,
        finished: IndexSet<Project>,
    }

    impl ResearchManager {
        pub fn unlock(&mut self, project: Project) {
            if self.locked.remove(&project) {
                let success = self.available.insert(project);
                assert!(success);
            }
        }

        pub fn new() -> ResearchManager {
            let initial = Project::BetterGuidelines1;
            let mut locked: IndexSet<_> = ALL_PROJECTS.into();
            let success = locked.remove(&initial);
            assert!(success);
            Self {
                locked,
                active: None,
                available: [initial].into(),
                finished: Default::default(),
            }
        }

        pub fn available(&self) -> &IndexSet<Project> {
            &self.available
        }

        pub fn active(&self) -> &Option<(Project, Quantity<ResearchPoints>)> {
            &self.active
        }

        pub fn activate(&mut self, project: Project) {
            assert!(self.active.is_none());
            let success = self.available.remove(&project);
            assert!(success);
            self.active = Some((project, Quantity::default()));
        }
    }
    impl World {
        pub fn simulate_research_manager(&mut self, delta: Duration) {
            let done_project =
                if let Some((project, progress)) = &mut self.cards.research.manager.active {
                    *progress += self.cards.research.rate * delta;
                    if progress >= &mut project.cost() {
                        Some(*project)
                    } else {
                        None
                    }
                } else {
                    None
                };
            if let Some(project) = done_project {
                project.apply(self);
                let success = self.cards.research.manager.finished.insert(project);
                assert!(success);
                self.cards.research.manager.active = None;
            }
        }
    }
}

pub struct Research {
    discovered: bool,
    manager: ResearchManager,
    rate: Rate<ResearchPoints>,
}

impl Research {
    pub fn new() -> Research {
        let mut initial_open = IndexSet::new();
        initial_open.insert(Project::BetterGuidelines1);

        Research {
            discovered: false,
            manager: ResearchManager::new(),
            rate: Rate::new(Quantity::new(1), Duration::MINUTE),
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
    fn render_inactive_not_empty(&mut self, input: &Input, mut view: MutGridView<'_, Cell>) {
        assert!(self.cards.research.manager.active().is_none());
        assert!(!self.cards.research.manager.available().is_empty());
        view.print_overflowing(0, "Choose next research project.".to_owned().into());

        let mut selected = None;
        for (mut id, &project) in self.cards.research.manager.available().iter().enumerate() {
            id += 1;
            view.print_overflowing(
                id,
                Text::new().raw(&format!(
                    "{}: {} [{}]",
                    id,
                    project.name(),
                    Duration::from_quantity_and_rate_approximation(
                        project.cost(),
                        self.cards.research.rate
                    )
                    .stringify(2)
                )),
            );

            let required_key = Key::number(id);
            if let Some(Event::Key(key)) = input.event {
                if key == required_key {
                    selected = Some(project);
                }
            }
        }

        if let Some(selected) = selected {
            self.cards.research.manager.activate(selected);
        }
    }

    fn render_inactive_empty(&mut self, _input: &Input, mut view: MutGridView<'_, Cell>) {
        view.print_overflowing(1, "No open research projects.".to_owned().into());
        view.print_overflowing(2, "Maybe come back later?".to_owned().into());
    }

    fn render_inactive(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        if self.cards.research.manager.available().is_empty() {
            self.render_inactive_empty(input, view);
        } else {
            self.render_inactive_not_empty(input, view);
        }
    }

    fn render_active(&mut self, _input: &Input, mut view: MutGridView<'_, Cell>) {
        let (project, progress) = self.cards.research.manager.active().unwrap();

        let dur = Duration::from_quantity_and_rate_approximation(
            project.cost() - progress,
            self.cards.research.rate,
        );

        view.print_overflowing(0, "Currently researching:".to_owned().into());
        view.print_overflowing(1, project.name().into());
        view.print_overflowing(
            3,
            format!(
                "Progress: {:.2}% [{}] ((({}s)))",
                100.0 * progress.as_f64() / project.cost().as_f64(),
                dur.stringify(2),
                dur.as_millis() / 1000.
            )
            .into(),
        );
    }

    pub(super) fn render_card_research(&mut self, input: &Input, view: MutGridView<'_, Cell>) {
        if self.cards.research.manager.active().is_some() {
            self.render_active(input, view);
        } else {
            self.render_inactive(input, view);
        }
    }

    pub(super) fn simulate_card_research(&mut self, delta: Duration) {
        if !self.cards.research.discovered
            && self.cards.activism.supporting_people >= Quantity::new(4)
        {
            self.cards.research.discovered = true;
            self.messages.queue(Message::new(
                "Research unlocked.".into(),
                STANDARD_MESSAGE_DURATION,
            ))
        }

        self.simulate_research_manager(delta);
    }
}
