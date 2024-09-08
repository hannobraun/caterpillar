use std::mem;

use crate::{
    breakpoints::Breakpoints, evaluator::Evaluator,
    instructions::InstructionAddress, Effect, Effects, Instructions, Stack,
    Value,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Process {
    most_recent_step: Option<InstructionAddress>,
    effects: Effects,
    evaluator: Evaluator,
    breakpoints: Breakpoints,
}

impl Process {
    pub fn most_recent_step(&self) -> Option<InstructionAddress> {
        self.most_recent_step
    }

    pub fn can_step(&self) -> bool {
        !self.has_finished() && self.inspect_effect().is_none()
    }

    pub fn has_finished(&self) -> bool {
        self.evaluator.stack.no_frames_left()
    }

    /// # Trigger the provided effect
    ///
    /// This must not be called, while an effect is already triggered. Only call
    /// it from contexts, where it's known that no effect could be triggered, or
    /// right after handling the currently triggered effect.
    ///
    /// ## Panics
    ///
    /// Panics, if an effect is already triggered.
    pub fn trigger_effect(&mut self, effect: impl Into<Effect>) {
        self.effects.trigger(effect)
    }

    /// # Inspect the triggered effect
    pub fn inspect_effect(&self) -> Option<&Effect> {
        self.effects.queue.front()
    }

    /// # Handle the triggered effect
    ///
    /// Returns the currently triggered effect, and considers it handled. Right
    /// after this call, subsequent calls to [`Effects::handle`] or
    /// [`Effects::inspect`] will return `None`.
    ///
    /// This method can safely be called, if no effect is currently triggered.
    /// In that case, it returns `None` and has no effect.
    pub fn handle_effect(&mut self) -> Option<Effect> {
        self.effects.queue.pop_front()
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

    pub fn breakpoints_mut(&mut self) -> &mut Breakpoints {
        &mut self.breakpoints
    }

    pub fn reset(&mut self, arguments: impl IntoIterator<Item = Value>) {
        // All we need to preserve when we reset are the breakpoints. Anything
        // else needs to go back to start conditions.
        //
        // Doing it like this, as opposed to just resetting all other fields,
        // has the advantage that this code doesn't need to be changed in sync
        // with new fields being added.
        let breakpoints = mem::take(&mut self.breakpoints);
        *self = Self {
            breakpoints,
            ..Self::default()
        };

        for argument in arguments {
            self.evaluator.stack.push_operand(argument);
        }
    }

    pub fn continue_(&mut self, and_stop_at: Option<InstructionAddress>) {
        if let Some(Effect::Breakpoint) = self.inspect_effect() {
            if let Some(address) = and_stop_at {
                self.breakpoints.set_ephemeral(address);
            }

            self.handle_effect();
        }
    }

    pub fn stop(&mut self) {
        self.breakpoints
            .set_ephemeral(self.evaluator.next_instruction);
    }

    pub fn step(&mut self, instructions: &Instructions) {
        if !self.can_step() {
            return;
        }

        let next_instruction = self.evaluator.next_instruction;

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&next_instruction)
        {
            self.trigger_effect(Effect::Breakpoint);
            return;
        }

        if let Err(effect) = self.evaluator.step(instructions) {
            self.trigger_effect(effect);
        }

        self.most_recent_step = Some(next_instruction);
    }
}
