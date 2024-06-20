use crate::runtime;

use super::{state::State, Event};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Breakpoints {
    state: State,
}

impl Breakpoints {
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn set_durable(&mut self, location: runtime::Location) {
        self.emit_event(Event::SetDurable { location });
    }

    pub fn clear_durable(&mut self, location: runtime::Location) {
        self.emit_event(Event::ClearDurable { location });
    }

    pub fn set_ephemeral(&mut self, location: runtime::Location) {
        self.emit_event(Event::SetEphemeral { location });
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        location: runtime::Location,
    ) -> bool {
        let durable_at_location = self.state.durable_at(&location);
        let ephemeral_at_location = self.state.ephemeral_at(&location);

        if ephemeral_at_location {
            self.emit_event(Event::ClearEphemeral { location });
        }

        ephemeral_at_location || durable_at_location
    }

    fn emit_event(&mut self, event: Event) {
        self.state.evolve(event);
    }
}
