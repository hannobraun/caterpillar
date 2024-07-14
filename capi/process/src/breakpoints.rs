use std::collections::BTreeSet;

use crate::{instructions::InstructionIndex, Location};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    durable: BTreeSet<InstructionIndex>,
    ephemeral: BTreeSet<InstructionIndex>,
}

impl Breakpoints {
    pub fn durable_at(&self, index: &InstructionIndex) -> bool {
        self.durable.contains(index)
    }

    pub fn ephemeral_at(&self, index: &InstructionIndex) -> bool {
        self.ephemeral.contains(index)
    }

    pub fn set_durable(&mut self, index: InstructionIndex) {
        self.durable.insert(index);
    }

    pub fn clear_durable(&mut self, index: &InstructionIndex) {
        self.durable.remove(index);
    }

    pub fn set_ephemeral(&mut self, index: InstructionIndex) {
        self.ephemeral.insert(index);
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        location: &Location,
    ) -> bool {
        let durable_at_location = self.durable_at(&location.index);
        let ephemeral_at_location = self.ephemeral_at(&location.index);

        if ephemeral_at_location {
            self.ephemeral.remove(&location.index);
        }

        ephemeral_at_location || durable_at_location
    }
}
