use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub has_finished: bool,
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
            Event::Finish => {
                self.has_finished = true;
            }
        }
    }
}
