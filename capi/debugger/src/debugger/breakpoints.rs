use std::collections::BTreeMap;

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
    ) -> Instruction {
        self.durable.remove(address).expect(
            "This method must not be called with an address that does not mark \
            a current breakpoint.",
        )
    }
}
