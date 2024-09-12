use anyhow::anyhow;
use capi_compiler::fragments::FragmentId;
use capi_game_engine::memory::Memory;
use capi_process::{Breakpoints, ProcessState, Value};
use capi_protocol::{
    command::CommandToRuntime,
    runtime_state::RuntimeState,
    updates::{Code, UpdateFromRuntime},
};

use super::{Action, ActiveFunctions};

#[derive(Clone, Debug, Default)]
pub struct PersistentState {
    pub code: Option<Code>,
    pub breakpoints: Breakpoints,
    pub runtime_state: Option<RuntimeState>,
    pub memory: Option<Memory>,
}

impl PersistentState {
    pub fn on_update_from_runtime(&mut self, update: UpdateFromRuntime) {
        match update {
            UpdateFromRuntime::Memory { memory } => {
                self.memory = Some(memory);
            }
            UpdateFromRuntime::Process(process) => {
                let runtime_state = match process.state() {
                    ProcessState::Running => RuntimeState::Running,
                    ProcessState::Finished => RuntimeState::Finished,
                    ProcessState::Stopped => RuntimeState::Stopped {
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
                    },
                };

                self.runtime_state = Some(runtime_state);
            }
        }
    }

    pub fn on_ui_action(
        &mut self,
        action: Action,
    ) -> anyhow::Result<Option<CommandToRuntime>> {
        let command = match action {
            Action::BreakpointClear { fragment, address } => {
                self.clear_durable_breakpoint(&fragment)?;

                Some(CommandToRuntime::BreakpointClear {
                    instruction: address,
                })
            }
            Action::BreakpointSet { fragment, address } => {
                self.set_durable_breakpoint(&fragment)?;

                Some(CommandToRuntime::BreakpointSet {
                    instruction: address,
                })
            }
            Action::Continue => Some(CommandToRuntime::Continue),
            Action::Reset => Some(CommandToRuntime::Reset),
            Action::Step => Some(CommandToRuntime::Step),
            Action::Stop => Some(CommandToRuntime::Stop),
        };

        Ok(command)
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
