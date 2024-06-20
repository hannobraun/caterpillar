use std::collections::BTreeSet;

use crate::runtime;

use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Breakpoints {
    durable: BTreeSet<runtime::Location>,
    ephemeral: BTreeSet<runtime::Location>,
}

impl Breakpoints {
    pub fn durable_at(&self, location: &runtime::Location) -> bool {
        self.durable.contains(location)
    }

    pub fn ephemeral_at(&self, location: &runtime::Location) -> bool {
        self.ephemeral.contains(location)
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
        let durable_at_location = self.durable_at(&location);
        let ephemeral_at_location = self.ephemeral_at(&location);

        if ephemeral_at_location {
            self.emit_event(Event::ClearEphemeral { location });
        }

        ephemeral_at_location || durable_at_location
    }

    fn emit_event(&mut self, event: Event) {
        self.evolve(event);
    }

    fn evolve(&mut self, event: Event) {
        match event {
            Event::SetDurable { location } => {
                self.durable.insert(location);
            }
            Event::ClearDurable { location } => {
                self.durable.remove(&location);
            }
            Event::SetEphemeral { location } => {
                self.ephemeral.insert(location);
            }
            Event::ClearEphemeral { location } => {
                self.ephemeral.remove(&location);
            }
        }
    }
}
