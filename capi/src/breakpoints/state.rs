use std::collections::BTreeSet;

use crate::runtime;

use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    durable: BTreeSet<runtime::Location>,
    ephemeral: BTreeSet<runtime::Location>,
}

impl State {
    pub fn evolve(&mut self, event: Event) {
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

    pub fn durable_at(&self, location: &runtime::Location) -> bool {
        self.durable.contains(location)
    }

    pub fn ephemeral_at(&self, location: &runtime::Location) -> bool {
        self.ephemeral.contains(location)
    }
}
