use anyhow::anyhow;
use capi_compiler::fragments::FragmentId;
use capi_game_engine::memory::Memory;
use capi_process::{Process, Value};
use capi_protocol::updates::{Code, UpdateFromRuntime};

use super::{ActiveFunctions, Breakpoints};

#[derive(Clone, Debug, Default)]
pub struct PersistentState {
    pub code: Option<Code>,
    pub breakpoints: Breakpoints,
    pub process: Option<Process>,
    pub memory: Option<Memory>,
}

impl PersistentState {
    pub fn on_update_from_runtime(&mut self, update: UpdateFromRuntime) {
        match update {
            UpdateFromRuntime::Memory { memory } => {
                self.memory = Some(memory);
            }
            UpdateFromRuntime::Process(process) => {
                self.process = Some(process);
            }
        }
    }

    pub fn generate_transient_state(&self) -> TransientState {
        TransientState {
            active_functions: ActiveFunctions::new(
                self.code.as_ref(),
                self.process.as_ref(),
            ),
            operands: self
                .process
                .as_ref()
                .map(|process| {
                    process
                        .stack()
                        .operands_in_current_stack_frame()
                        .copied()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        }
    }

    pub fn set_durable_breakpoint(
        &mut self,
        fragment: &FragmentId,
    ) -> anyhow::Result<()> {
        let code = self
            .code
            .as_ref()
            .ok_or_else(|| anyhow!("Code is not available yet."))?;
        let address = code
            .source_map
            .fragment_to_instructions(fragment)
            .first()
            .copied()
            .ok_or_else(|| anyhow!("Fragment does not map to instruction."))?;
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
        fragment: &FragmentId,
    ) -> anyhow::Result<()> {
        let code = self
            .code
            .as_ref()
            .ok_or_else(|| anyhow!("Code is not available yet."))?;
        let address = code
            .source_map
            .fragment_to_instructions(fragment)
            .first()
            .ok_or_else(|| anyhow!("Fragment does not map to instruction."))?;

        self.breakpoints.clear_durable(address);

        Ok(())
    }
}

#[derive(Clone)]
pub struct TransientState {
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
}
