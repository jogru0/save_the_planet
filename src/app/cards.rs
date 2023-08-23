use ratatui::style::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Card {
    Flyer,
    Achievements,
}

impl Card {
    pub fn color(&self) -> Color {
        match self {
            Card::Flyer => Color::Cyan,
            Card::Achievements => Color::Yellow,
        }
    }

    pub fn text(&self) -> String {
        match self {
            Card::Flyer => "Flyer".to_owned(),
            Card::Achievements => "Achievements".to_owned(),
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Card::Flyer => Some(Card::Achievements),
            Card::Achievements => None,
        }
    }

    pub fn previous(&self) -> Option<Self> {
        match self {
            Card::Flyer => None,
            Card::Achievements => Some(Card::Flyer),
        }
    }
}

#[derive(Debug)]
pub struct Cards {
    pub flyer: Flyer,
    pub achievements: Option<Achievements>,

    pub selected: Card,
}

impl Cards {
    pub fn new() -> Self {
        Self {
            flyer: Flyer::new(),
            achievements: None,
            selected: Card::Flyer,
        }
    }

    pub(crate) fn next(&mut self) {
        let mut new = self.selected;

        loop {
            match new.next() {
                None => return,
                Some(next) => {
                    if self.contains(next) {
                        self.selected = next;
                        return;
                    } else {
                        new = next
                    }
                }
            }
        }
    }

    pub(crate) fn previous(&mut self) {
        let mut new = self.selected;

        loop {
            match new.previous() {
                None => return,
                Some(previous) => {
                    if self.contains(previous) {
                        self.selected = previous;
                        return;
                    } else {
                        new = previous
                    }
                }
            }
        }
    }

    fn contains(&self, card: Card) -> bool {
        match card {
            Card::Flyer => true,
            Card::Achievements => self.achievements.is_some(),
        }
    }
}

impl Default for Cards {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Flyer {
    pub saved_co2: u128,
    pub available_flyers: Option<u128>,
}

impl Flyer {
    fn new() -> Self {
        Self {
            saved_co2: 0,
            available_flyers: None,
        }
    }
}

#[derive(Debug)]
pub struct Achievements {
    pub unlocked: Vec<Achievement>,
}

impl Achievements {
    pub fn new() -> Self {
        Self {
            unlocked: vec![Achievement {
                text: "Unlock achievements.".to_owned(),
            }],
        }
    }
}

impl Default for Achievements {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Achievement {
    pub text: String,
}
