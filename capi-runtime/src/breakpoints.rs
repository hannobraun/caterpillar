use std::collections::BTreeSet;

use crate::runtime;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    durable: BTreeSet<runtime::InstructionAddress>,
    ephemeral: BTreeSet<runtime::InstructionAddress>,
}

impl Breakpoints {
    pub fn toggle_durable_at(&mut self, location: runtime::InstructionAddress) {
        if self.durable.take(&location).is_none() {
            self.durable.insert(location);
        }
    }

    pub fn set_ephemeral(&mut self, location: runtime::InstructionAddress) {
        self.ephemeral.insert(location);
    }

    pub fn durable_breakpoint_at(
        &self,
        location: &runtime::InstructionAddress,
    ) -> bool {
        self.durable.contains(location)
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        location: &runtime::InstructionAddress,
    ) -> bool {
        let ephemeral = self.ephemeral.take(location).is_some();
        ephemeral || self.durable_breakpoint_at(location)
    }
}
