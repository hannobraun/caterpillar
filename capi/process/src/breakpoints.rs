use std::collections::BTreeSet;

use crate::instructions::InstructionAddress;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    durable: BTreeSet<InstructionAddress>,
    ephemeral: BTreeSet<InstructionAddress>,
}

impl Breakpoints {
    pub fn durable_at(&self, instruction: &InstructionAddress) -> bool {
        self.durable.contains(instruction)
    }

    pub fn ephemeral_at(&self, instruction: &InstructionAddress) -> bool {
        self.ephemeral.contains(instruction)
    }

    pub fn set_durable(&mut self, instruction: InstructionAddress) {
        self.durable.insert(instruction);
    }

    pub fn clear_durable(&mut self, instruction: &InstructionAddress) {
        self.durable.remove(instruction);
    }

    pub fn set_ephemeral(&mut self, instruction: InstructionAddress) {
        self.ephemeral.insert(instruction);
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        instruction: &InstructionAddress,
    ) -> bool {
        let should_stop =
            self.ephemeral_at(instruction) || self.durable_at(instruction);

        if self.ephemeral_at(instruction) {
            // The debugger sets ephemeral breakpoints to implement its "Step"
            // functionality. It might only set one such breakpoint, that needs
            // to be cleared once it's hit.
            //
            // But if the next step is into a branch, the debugger might not
            // know where exactly it's going to go, and might set breakpoints
            // into all of the branches.
            //
            // We don't want those breakpoints to hang around, to stop execution
            // at some later point. Therefore, we clear all of them when a
            // single one is hit.
            self.ephemeral.clear();
        }

        should_stop
    }
}
