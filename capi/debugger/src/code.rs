use std::collections::BTreeMap;

use anyhow::anyhow;
use capi_process::{Instruction, InstructionAddress, Instructions};
use capi_protocol::updates::Code;
use tokio::sync::watch;

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

#[derive(Clone, Debug, Default)]
pub struct DebugCode {
    pub inner: Option<Code>,
}

impl DebugCode {
    pub fn update(&mut self, code: Code) {
        self.inner = Some(code);
    }
}

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
