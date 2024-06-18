use crate::runtime;

use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    most_recent_step: Option<runtime::Location>,
    has_finished: bool,
}

impl State {
    pub fn is_running(&self) -> bool {
        !self.has_finished
    }

    pub fn has_finished(&self) -> bool {
        self.has_finished
    }

    pub fn evolve(&mut self, event: Event) {
        match event {
            Event::MostRecentStep { location } => {
                self.most_recent_step = Some(location);
            }
            Event::Finish => {
                self.has_finished = true;
            }
        }
    }
}
