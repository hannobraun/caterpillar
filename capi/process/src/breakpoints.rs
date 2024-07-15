use std::collections::BTreeSet;

use crate::instructions::InstructionAddr;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    durable: BTreeSet<InstructionAddr>,
    ephemeral: BTreeSet<InstructionAddr>,
}

impl Breakpoints {
    pub fn durable_at(&self, instruction: &InstructionAddr) -> bool {
        self.durable.contains(instruction)
    }

    pub fn ephemeral_at(&self, index: &InstructionAddr) -> bool {
        self.ephemeral.contains(index)
    }

    pub fn set_durable(&mut self, index: InstructionAddr) {
        self.durable.insert(index);
    }

    pub fn clear_durable(&mut self, index: &InstructionAddr) {
        self.durable.remove(index);
    }

    pub fn set_ephemeral(&mut self, index: InstructionAddr) {
        self.ephemeral.insert(index);
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        index: &InstructionAddr,
    ) -> bool {
        let durable_at_location = self.durable_at(index);
        let ephemeral_at_location = self.ephemeral_at(index);

        if ephemeral_at_location {
            self.ephemeral.remove(index);
        }

        ephemeral_at_location || durable_at_location
    }
}
