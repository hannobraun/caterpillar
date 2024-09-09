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

    /// # Clear a durable breakpoint
    ///
    /// Must only be called with an `address` argument that designates a
    /// currently active durable breakpoint. Meaning, and address for which a
    /// breakpoint has previously been set, using [`Breakpoints::set_durable`],
    /// without having been cleared since, using this method.
    ///
    /// Returns the instruction that was previously passed to `set_durable`.
    ///
    /// ## Panics
    ///
    /// Panics, if `address` does not designate a currently active breakpoint.
    pub fn clear_durable(
        &mut self,
        address: &InstructionAddress,
    ) -> anyhow::Result<Instruction> {
        self.durable
            .remove(address)
            .ok_or_else(|| anyhow!("No breakpoint at `{address}`"))
    }
}
