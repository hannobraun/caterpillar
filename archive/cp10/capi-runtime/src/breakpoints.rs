use std::collections::BTreeSet;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    durable: BTreeSet<InstructionAddress>,
    ephemeral: BTreeSet<InstructionAddress>,
}

impl Breakpoints {
    pub fn toggle_durable_at(&mut self, address: InstructionAddress) {
        if self.durable.take(&address).is_none() {
            self.durable.insert(address);
        }
    }

    pub fn set_ephemeral(&mut self, address: InstructionAddress) {
        self.ephemeral.insert(address);
    }

    pub fn durable_breakpoint_at(&self, address: &InstructionAddress) -> bool {
        self.durable.contains(address)
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        address: &InstructionAddress,
    ) -> bool {
        let ephemeral = self.ephemeral.take(address).is_some();
        ephemeral || self.durable_breakpoint_at(address)
    }
}
