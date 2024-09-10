use std::collections::BTreeMap;

use anyhow::anyhow;
use capi_process::{Instruction, InstructionAddress, Instructions};
use capi_protocol::updates::Code;
use tokio::sync::watch;

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

#[derive(Clone, Debug, Default)]
pub struct DebugCode {
    pub code_from_server: Option<Code>,
    pub breakpoints: Breakpoints,
}

impl DebugCode {
    pub fn update(&mut self, code: Code) {
        self.code_from_server = Some(code);
    }

    pub fn set_durable_breakpoint(
        &mut self,
        address: InstructionAddress,
    ) -> anyhow::Result<()> {
        let code = self
            .code_from_server
            .as_ref()
            .ok_or_else(|| anyhow!("Code is not available yet."))?;
        let instruction = code
            .instructions
            .get(&address)
            .ok_or_else(|| {
                anyhow!("Instruction at `{address}` does not exist.")
            })?
            .clone();

        self.breakpoints.set_durable(address, instruction);

        Ok(())
    }

    pub fn clear_durable_breakpoint(
        &mut self,
        address: &InstructionAddress,
    ) -> anyhow::Result<()> {
        self.breakpoints.clear_durable(address)?;
        Ok(())
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
