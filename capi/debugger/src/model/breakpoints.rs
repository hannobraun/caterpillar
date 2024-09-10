use std::collections::BTreeMap;

use anyhow::anyhow;
use capi_process::{Instruction, InstructionAddress};

#[derive(Clone, Debug, Default)]
pub struct Breakpoints {
    durable: BTreeMap<InstructionAddress, Instruction>,
}

impl Breakpoints {
    pub fn set_durable(
        &mut self,
        address: InstructionAddress,
        instruction: Instruction,
    ) {
        self.durable.insert(address, instruction);
    }

    pub fn clear_durable(
        &mut self,
        address: &InstructionAddress,
    ) -> anyhow::Result<Instruction> {
        self.durable
            .remove(address)
            .ok_or_else(|| anyhow!("No breakpoint at `{address}`"))
    }
}
