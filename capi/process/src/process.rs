use std::mem;

use crate::{
    breakpoints::Breakpoints, evaluator::Evaluator,
    instructions::InstructionAddress, Command, Effect, Effects, Instructions,
    Stack, Value,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Process {
    arguments: Vec<Value>,
    effects: Effects,
    evaluator: Evaluator,
    breakpoints: Breakpoints,
}

impl Process {
    pub fn new(arguments: impl IntoIterator<Item = Value>) -> Self {
        let mut self_ = Self {
            arguments: arguments.into_iter().collect(),
            effects: Effects::default(),
            evaluator: Evaluator::default(),
            breakpoints: Breakpoints::default(),
        };

        self_.reset();

        self_
    }

    pub fn state(&self) -> ProcessState {
        if self.effects.inspect_first().is_some() {
            ProcessState::Stopped
        } else if self.evaluator.stack.no_frames_left() {
            ProcessState::Finished
        } else {
            ProcessState::Running
        }
    }

    pub fn effects(&self) -> &Effects {
        &self.effects
    }

    pub fn effects_mut(&mut self) -> &mut Effects {
        &mut self.effects
    }

    pub fn evaluator(&self) -> &Evaluator {
        &self.evaluator
    }

    pub fn stack(&self) -> &Stack {
        &self.evaluator.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.evaluator.stack
    }

    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    pub fn on_command(&mut self, command: Command) {
        match command {
            Command::BreakpointClear { instruction } => {
                self.breakpoints.clear_durable(&instruction);
            }
            Command::BreakpointSet { instruction } => {
                self.breakpoints.set_durable(instruction);
            }
            Command::Continue => {
                self.continue_(None);
            }
            Command::Reset => {
                self.reset();
            }
            Command::Step => {
                if let Some(Effect::Breakpoint) = self.effects.inspect_first() {
                    let and_stop_at = self.evaluator.next_instruction;
                    self.continue_(Some(and_stop_at))
                } else {
                    // If we're not stopped at a breakpoint, we can't step.
                    // It would be better, if this resulted in an explicit
                    // error that is sent to the debugger, instead of
                    // silently being ignored here.
                }
            }
            Command::Stop => {
                self.effects.trigger(Effect::Breakpoint);
            }
        }
    }

    fn reset(&mut self) {
        // There are some fields we need to preserve over the reset. Anything
        // else needs to go back to start conditions.
        //
        // Doing it like this, as opposed to just resetting all other fields,
        // has the advantage that this code doesn't need to be changed in sync
        // with new fields being added.
        let arguments = mem::take(&mut self.arguments);
        let breakpoints = mem::take(&mut self.breakpoints);

        *self = Self {
            arguments,
            breakpoints,

            effects: Effects::default(),
            evaluator: Evaluator::default(),
        };

        for argument in &self.arguments {
            self.evaluator.stack.push_operand(*argument);
        }
    }

    fn continue_(&mut self, and_stop_at: Option<InstructionAddress>) {
        if let Some(Effect::Breakpoint) = self.effects.inspect_first() {
            if let Some(address) = and_stop_at {
                self.breakpoints.set_ephemeral(address);
            }

            self.effects.handle_first();
        }
    }

    pub fn evaluate_next_instruction(&mut self, instructions: &Instructions) {
        if !self.state().is_running() {
            return;
        }

        let next_instruction = self.evaluator.next_instruction;

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&next_instruction)
        {
            self.effects.trigger(Effect::Breakpoint);
            return;
        }

        if let Err(effect) = self.evaluator.step(instructions) {
            self.effects.trigger(effect);
        }
    }

    /// # Ignore the next instruction
    ///
    /// This advances the instruction that the evaluator is going to evaluate
    /// next by one This must be done after a host effect has been handled.
    ///
    /// When an instruction triggers an effect, the evaluator does not advance
    /// to the next instruction. This makes breakpoints work as expected: Once
    /// the breakpoint effect triggers, the developer still has the opportunity
    /// to step into the current instruction; as opposed to that instruction
    /// already being in the past.
    ///
    /// This behavior also provides the opportunity for recovering from an
    /// error, by updating the instruction that triggered it and then re-trying.
    ///
    /// But after a host effect has been handled, the instruction that triggered
    /// it must not be executed again. This can be ensured by calling this
    /// method after the host effect has been handled.
    ///
    /// ## Implementation Note
    ///
    /// Leaving this important responsibility to the host, which means _every_
    /// host implementation must remember to call this function at the
    /// appropriate time, is a questionable design decision.
    ///
    /// For now, I (Hanno Braun) figured that it's best to keep effects simple
    /// and uniform to see how the overall design shakes out, instead of
    /// burdening it with complications (like handling host effects specially)
    /// from early on.
    pub fn ignore_next_instruction(&mut self) {
        self.evaluator.next_instruction =
            self.evaluator.next_instruction.next();
    }
}

pub enum ProcessState {
    Running,
    Finished,
    Stopped,
}

impl ProcessState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn has_finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}
