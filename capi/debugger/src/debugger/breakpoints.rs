use std::collections::BTreeSet;

use capi_process::InstructionAddress;

#[derive(Clone, Debug, Default)]
pub struct Breakpoints {
    durable: BTreeSet<InstructionAddress>,
}

impl Breakpoints {
    pub fn set_durable(&mut self, instruction: InstructionAddress) {
        self.durable.insert(instruction);
    }

    pub fn clear_durable(&mut self, instruction: &InstructionAddress) {
        self.durable.remove(instruction);
    }
}
