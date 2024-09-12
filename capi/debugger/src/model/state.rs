use anyhow::anyhow;
use capi_compiler::fragments::FragmentId;
use capi_game_engine::memory::Memory;
use capi_process::{Breakpoints, Process, Value};
use capi_protocol::{
    runtime_state::RuntimeState,
    updates::{Code, UpdateFromRuntime},
};

use super::ActiveFunctions;

#[derive(Clone, Debug, Default)]
pub struct PersistentState {
    pub code: Option<Code>,
    pub breakpoints: Breakpoints,
    pub runtime_state: Option<RuntimeState>,
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
                let runtime_state = if process.has_finished() {
                    RuntimeState::Finished
                } else if process.can_step() {
                    RuntimeState::Running
                } else {
                    RuntimeState::Stopped {
                        effects: process.effects().queue().collect(),
                        active_instructions: process
                            .evaluator()
                            .active_instructions()
                            .collect(),
                        current_operands: process
                            .stack()
                            .operands_in_current_stack_frame()
                            .copied()
                            .collect::<Vec<_>>(),
                    }
                };

                self.runtime_state = Some(runtime_state);
                self.process = Some(process);
            }
        }
    }

    pub fn generate_transient_state(&self) -> TransientState {
        let active_functions = ActiveFunctions::new(
            self.code.as_ref(),
            &self.breakpoints,
            self.runtime_state.as_ref(),
        );
        let operands = match &self.runtime_state {
            Some(RuntimeState::Stopped {
                current_operands, ..
            }) => current_operands.clone(),
            _ => Vec::new(),
        };

        TransientState {
            active_functions,
            operands,
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

        self.breakpoints.set_durable(address);

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
