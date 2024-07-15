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

    pub fn ephemeral_at(&self, instruction: &InstructionAddr) -> bool {
        self.ephemeral.contains(instruction)
    }

    pub fn set_durable(&mut self, instruction: InstructionAddr) {
        self.durable.insert(instruction);
    }

    pub fn clear_durable(&mut self, instruction: &InstructionAddr) {
        self.durable.remove(instruction);
    }

    pub fn set_ephemeral(&mut self, instruction: InstructionAddr) {
        self.ephemeral.insert(instruction);
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
