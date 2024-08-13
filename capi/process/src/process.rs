use std::{collections::VecDeque, mem};

use crate::{
    breakpoints::Breakpoints, evaluator::Evaluator,
    instructions::InstructionAddress, Effect, Instructions, Stack, Value,
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
        !self.has_finished() && self.effects.queue.is_empty()
    }

    pub fn has_finished(&self) -> bool {
        self.evaluator.stack.no_frames_left()
    }

    pub fn effects(&self) -> &Effects {
        &self.effects
    }

    pub fn effects_mut(&mut self) -> &mut Effects {
        &mut self.effects
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

    pub fn clear_durable_breakpoint(
        &mut self,
        instruction: &InstructionAddress,
    ) {
        self.breakpoints.clear_durable(instruction);
    }

    pub fn set_durable_breakpoint(&mut self, instruction: InstructionAddress) {
        self.breakpoints.set_durable(instruction);
    }

    pub fn continue_(&mut self, and_stop_at: Option<InstructionAddress>) {
        if let Some(Effect::Breakpoint) = self.effects.first() {
            if let Some(instruction) = and_stop_at {
                self.breakpoints.set_ephemeral(instruction);
            }

            self.effects.handle_first();
        }
    }

    pub fn stop(&mut self) {
        let next_instruction = self.stack().next_instruction();
        self.breakpoints.set_ephemeral(next_instruction);
    }

    pub fn step(&mut self, instructions: &Instructions) {
        if !self.can_step() {
            return;
        }

        let next_instruction = self.evaluator.stack.next_instruction();

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&next_instruction)
        {
            self.effects.trigger(Effect::Breakpoint);
        }

        if let Err(effect) = self.evaluator.step(instructions) {
            self.effects.trigger(effect);
        }

        self.most_recent_step = Some(next_instruction);
    }
}

/// The currently active effects of a process
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Effects {
    queue: VecDeque<Effect>,
}

impl Effects {
    /// Look at the first effect in the queue
    pub fn first(&self) -> Option<&Effect> {
        self.queue.front()
    }

    /// Handle the first effect in the queue
    ///
    /// If it can't be handled for some reason, which is probably a fatal
    /// failure, it should be re-triggered, to make sure all required
    /// information is available for debugging.
    pub fn handle_first(&mut self) -> Option<Effect> {
        self.queue.pop_front()
    }

    /// Trigger the provided effect
    ///
    /// The new effect is added to the front of the queue.
    pub fn trigger(&mut self, effect: impl Into<Effect>) {
        self.queue.push_front(effect.into());
    }
}
