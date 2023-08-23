#[derive(Debug)]
pub struct Cards {
    pub flyer: Flyer,
    pub achievements: Option<Achievements>,

    pub selected: String,
}

impl Cards {
    pub fn new() -> Self {
        Self {
            flyer: Flyer::new(),
            achievements: None,
            selected: "Flyer".to_string(),
        }
    }

    pub(crate) fn next(&mut self) {
        if self.selected == "Flyer" && self.achievements.is_some() {
            self.selected = "Achievements".to_owned();
        }
    }

    pub(crate) fn previous(&mut self) {
        self.selected = String::from("Flyer")
    }
}

impl Default for Cards {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Flyer {
    count: u128,
    buyer: u128,
}

impl Flyer {
    fn new() -> Self {
        Self { count: 0, buyer: 0 }
    }
}

#[derive(Debug)]
pub struct Achievements {
    unlocked: Vec<Achievement>,
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

#[derive(Debug)]
struct Achievement {
    text: String,
}
