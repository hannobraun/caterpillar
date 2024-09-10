use anyhow::anyhow;
use capi_game_engine::memory::Memory;
use capi_process::{InstructionAddress, Process, Value};

use crate::code::{Breakpoints, DebugCode};

use super::ActiveFunctions;

#[derive(Clone, Debug, Default)]
pub struct Debugger {
    pub code: DebugCode,
    pub breakpoints: Breakpoints,
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
    pub memory: Option<Memory>,
}

impl Debugger {
    pub fn update(
        &mut self,
        memory: Option<Memory>,
        process: Option<&Process>,
    ) {
        self.active_functions =
            ActiveFunctions::new(self.code.code_from_server.as_ref(), process);
        self.operands = process
            .map(|process| {
                process
                    .stack()
                    .operands_in_current_stack_frame()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.memory = memory;
    }

    pub fn set_durable_breakpoint(
        &mut self,
        address: InstructionAddress,
    ) -> anyhow::Result<()> {
        let code = self
            .code
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
