use std::collections::BTreeSet;

use crate::runtime;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
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
        self.durable.insert(location);
    }

    pub fn clear_durable(&mut self, location: runtime::Location) {
        self.durable.remove(&location);
    }

    pub fn set_ephemeral(&mut self, location: runtime::Location) {
        self.ephemeral.insert(location);
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        location: runtime::Location,
    ) -> bool {
        let durable_at_location = self.durable_at(&location);
        let ephemeral_at_location = self.ephemeral_at(&location);

        if ephemeral_at_location {
            self.ephemeral.remove(&location);
        }

        ephemeral_at_location || durable_at_location
    }
}
