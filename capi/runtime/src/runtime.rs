use crate::{evaluator::Evaluator, Effects, Instructions, Stack, Value};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Runtime {
    effects: Effects,
    evaluator: Evaluator,
}

impl Runtime {
    pub fn state(&self) -> RuntimeState {
        if self.effects.inspect_first().is_some() {
            RuntimeState::Stopped
        } else if self.evaluator.stack.no_frames_left() {
            RuntimeState::Finished
        } else {
            RuntimeState::Running
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

    pub fn reset(&mut self, arguments: impl IntoIterator<Item = Value>) {
        *self = Self::default();

        for argument in arguments {
            self.evaluator.stack.push_operand(argument);
        }
    }

    pub fn evaluate_next_instruction(&mut self, instructions: &Instructions) {
        if !self.state().is_running() {
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

pub enum RuntimeState {
    Running,
    Finished,
    Stopped,
}

impl RuntimeState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn has_finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}
