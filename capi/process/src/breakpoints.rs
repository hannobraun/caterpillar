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
        instruction: &InstructionAddr,
    ) -> bool {
        let should_stop =
            self.ephemeral_at(instruction) || self.durable_at(instruction);

        if self.ephemeral_at(instruction) {
            self.ephemeral.remove(instruction);
        }

        should_stop
    }
}
