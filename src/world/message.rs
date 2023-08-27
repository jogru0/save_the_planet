pub struct Messages {
    entries: VecDeque<Message>,
    current_duration: Duration,
}
impl Messages {
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
            current_duration: Duration::INSTANT,
        }
    }

    pub fn queue(&mut self, message: Message) {
        self.entries.push_back(message);
    }

    pub fn simulate(&mut self, delta: Duration) {
        if let Some(entry) = self.entries.front() {
            self.current_duration += delta;
            if entry.duration <= self.current_duration {
                self.entries.pop_front();
                self.current_duration = Duration::INSTANT;
            }
        }
    }

    pub fn get_current(&self) -> Option<&Message> {
        self.entries.front()
    }
}

use std::collections::VecDeque;

use crate::duration::Duration;

pub const STANDARD_MESSAGE_DURATION: Duration = Duration::from_seconds(5);

pub struct Message {
    text: String,
    duration: Duration,
}

impl Message {
    pub fn new(text: String, duration: Duration) -> Self {
        assert_ne!(duration, Duration::INSTANT);
        Self { text, duration }
    }

    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}
