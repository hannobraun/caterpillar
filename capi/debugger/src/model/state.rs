use capi_compiler::fragments::FragmentId;
use capi_game_engine::{command::Command, memory::Memory};
use capi_process::{Effect, Instruction, Instructions, ProcessState, Value};
use capi_protocol::{
    runtime_state::RuntimeState,
    updates::{Code, UpdateFromRuntime},
};

use super::{
    ActiveFunctions, Breakpoints, DebugCode, DebugFragmentKind, UserAction,
};

#[derive(Clone, Debug, Default)]
pub struct PersistentState {
    pub code: DebugCode,
    pub breakpoints: Breakpoints,
    pub runtime_state: Option<RuntimeState>,
    pub memory: Option<Memory>,
}

impl PersistentState {
    pub fn on_new_code(&mut self, code: Code) -> Command {
        let instructions = self.apply_breakpoints(&code);
        self.code.inner = Some(code);
        Command::UpdateCode { instructions }
    }

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

    pub fn on_user_action(
        &mut self,
        action: UserAction,
        transient: &TransientState,
    ) -> anyhow::Result<Vec<Command>> {
        let mut commands = Vec::new();

        match action {
            UserAction::BreakpointClear { fragment, .. } => {
                let code = self.code.get()?;
                let address = self.code.fragment_to_instruction(&fragment)?;

                self.breakpoints.clear_durable(&address);

                commands.push(Command::UpdateCode {
                    instructions: self.apply_breakpoints(code),
                });
            }
            UserAction::BreakpointSet { fragment, .. } => {
                let code = self.code.get()?;
                let address = self.code.fragment_to_instruction(&fragment)?;

                self.breakpoints.set_durable(address);

                commands.push(Command::UpdateCode {
                    instructions: self.apply_breakpoints(code),
                });
            }
            UserAction::Continue => {
                let origin = transient
                    .active_functions
                    .entries()?
                    .leaf()
                    .function()?
                    .active_branch()?
                    .active_fragment()?
                    .id();
                let targets = Vec::new();

                self.step_or_continue(&origin, targets, &mut commands)?;
            }
            UserAction::Reset => {
                commands.push(Command::Reset);
            }
            UserAction::StepIn => {
                let code = self.code.get()?;

                let entries = transient.active_functions.entries()?;
                let branch = entries.leaf().function()?.active_branch()?;

                let origin = branch.active_fragment()?;
                let targets = if let Some(function) =
                    origin.data.fragment.as_call_to_function(&code.fragments)
                {
                    function
                        .branches
                        .iter()
                        .map(|branch| branch.start)
                        .collect()
                } else {
                    let mut fragment = origin.clone();

                    loop {
                        let Some(after) = entries
                            .find_next_fragment_or_next_after_caller(
                                branch,
                                &fragment.id(),
                            )?
                        else {
                            // Can't find a next fragment _or_ a caller, which
                            // means we must be at the top-level function.
                            //
                            // Let's just tell the runtime to continue, so the
                            // process finishes.
                            self.step_or_continue(
                                &origin.id(),
                                vec![],
                                &mut commands,
                            )?;
                            return Ok(commands);
                        };

                        if let DebugFragmentKind::Comment { .. } = after.kind {
                            // Can't step to comments! Need to ignore them.
                            fragment = after.clone();
                            continue;
                        }

                        break vec![after.id()];
                    }
                };

                self.step_or_continue(&origin.id(), targets, &mut commands)?;
            }
            UserAction::StepOut => {
                let entries = transient.active_functions.entries()?;
                let origin = entries
                    .leaf()
                    .function()?
                    .active_branch()?
                    .active_fragment()?;

                let targets = {
                    let mut fragment = origin.clone();

                    loop {
                        let Some(after) = entries
                            .find_next_fragment_after_caller(&fragment.id())?
                        else {
                            // Can't find a next fragment _or_ a caller, which
                            // means we must be at the top-level function.
                            //
                            // Let's just tell the runtime to continue, so the
                            // process finishes.
                            self.step_or_continue(
                                &origin.id(),
                                vec![],
                                &mut commands,
                            )?;
                            return Ok(commands);
                        };

                        if let DebugFragmentKind::Comment { .. } = after.kind {
                            // Can't step to comments! Need to ignore them.
                            fragment = after.clone();
                            continue;
                        }

                        break vec![after.id()];
                    }
                };

                self.step_or_continue(&origin.id(), targets, &mut commands)?;
            }
            UserAction::StepOver => {
                let entries = transient.active_functions.entries()?;
                let branch = entries.leaf().function()?.active_branch()?;

                let origin = branch.active_fragment()?;

                let targets = {
                    let mut fragment = origin.clone();

                    loop {
                        let Some(after) = entries
                            .find_next_fragment_or_next_after_caller(
                                branch,
                                &fragment.id(),
                            )?
                        else {
                            // Can't find a next fragment _or_ a caller, which
                            // means we must be at the top-level function.
                            //
                            // Let's just tell the runtime to continue, so the
                            // process finishes.
                            self.step_or_continue(
                                &origin.id(),
                                vec![],
                                &mut commands,
                            )?;
                            return Ok(commands);
                        };

                        if let DebugFragmentKind::Comment { .. } = after.kind {
                            // Can't step to comments! Need to ignore them.
                            fragment = after.clone();
                            continue;
                        }

                        break vec![after.id()];
                    }
                };

                self.step_or_continue(&origin.id(), targets, &mut commands)?;
            }
            UserAction::Stop => {
                commands.push(Command::Stop);
            }
        };

        Ok(commands)
    }

    pub fn generate_transient_state(&self) -> TransientState {
        let active_functions = ActiveFunctions::new(
            self.code.inner.as_ref(),
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

    fn step_or_continue(
        &mut self,
        origin: &FragmentId,
        targets: Vec<FragmentId>,
        commands: &mut Vec<Command>,
    ) -> anyhow::Result<()> {
        let code = self.code.get()?;

        let origin = self.code.fragment_to_instruction(origin)?;

        if let Instruction::TriggerEffect {
            effect: Effect::Breakpoint,
        } = self.code.instruction(&origin)?
        {
            // The instruction we're trying to step away from was compiled from
            // a `brk` instruction, or something equivalent. That won't ever do
            // anything except trigger another breakpoint.
            //
            // To handle this, let's first add a breakpoint at the next
            // instruction, to prepare for stepping over the `brk`.
            //
            // The next instruction address we're creating here should always be
            // valid. Even if the `brk` is the last fragment in a function,
            // there's always going to be a return instruction, at least.
            let mut instructions = self.apply_breakpoints(code);

            // Now we can temporarily replace the `brk` with a `nop`, which we
            // can step over.
            instructions.replace(origin, Instruction::Nop);

            // Everything's prepared to send the required commands now.
            commands.extend([
                Command::UpdateCode { instructions },
                Command::ClearBreakpointAndEvaluateNextInstruction,
            ]);

            // But we also need to reverse the change that we've made. Since we
            // re-apply the breakpoints based on the original code, we don't
            // need to do another replacement to get rid of the `nop`.
            commands.push(Command::UpdateCode {
                instructions: self.apply_breakpoints(code),
            });
        }

        if self.breakpoints.durable_at(&origin) {
            // We are currently stopped at a durable breakpoint. That's going to
            // require some special handling.
            //
            // First, clear the breakpoint temporarily.
            self.breakpoints.clear_durable(&origin);

            // Now that the breakpoint is cleared, send updated code to the
            // runtime and tell it to step beyond where the breakpoint was.
            commands.extend([
                Command::UpdateCode {
                    instructions: self.apply_breakpoints(code),
                },
                Command::ClearBreakpointAndEvaluateNextInstruction,
            ]);

            // Now that we're past it, we can but the breakpoint back.
            self.breakpoints.set_durable(origin);

            // And of course we need to send updated code to the runtime again,
            // or we risk it running beyond the breakpoint.
            commands.push(Command::UpdateCode {
                instructions: self.apply_breakpoints(code),
            });
        }

        self.breakpoints.clear_all_ephemeral();

        let targets = targets
            .into_iter()
            .map(|target| self.code.fragment_to_instruction(&target))
            .collect::<Result<Vec<_>, _>>()?;
        for target in targets {
            self.breakpoints.set_ephemeral(target);
        }

        commands.extend([
            Command::UpdateCode {
                instructions: self.apply_breakpoints(code),
            },
            Command::Continue,
        ]);

        Ok(())
    }

    fn apply_breakpoints(&self, code: &Code) -> Instructions {
        let mut instructions = code.instructions.clone();

        for address in self.breakpoints.iter() {
            instructions.replace(
                address,
                Instruction::TriggerEffect {
                    effect: Effect::Breakpoint,
                },
            );
        }

        instructions
    }
}

#[derive(Clone, Debug)]
pub struct TransientState {
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
}
