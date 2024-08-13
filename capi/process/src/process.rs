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
        !self.has_finished() && self.effects.unhandled_effects.is_empty()
    }

    pub fn has_finished(&self) -> bool {
        self.evaluator.stack.no_frames_left()
    }

    pub fn state(&self) -> &Effects {
        &self.effects
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

    pub fn handle_first_effect(&mut self) -> Option<Effect> {
        self.effects.unhandled_effects.pop_front()
    }

    /// Trigger the provided effect
    ///
    /// If there already is an unhandled effect, this new effect will displace
    /// it as the first effect, meaning the existing effect will be moved back
    /// in the list.
    pub fn trigger_effect(&mut self, effect: impl Into<Effect>) {
        self.effects.unhandled_effects.push_front(effect.into());
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
        if let Some(Effect::Breakpoint) = self.effects.first_unhandled_effect()
        {
            if let Some(instruction) = and_stop_at {
                self.breakpoints.set_ephemeral(instruction);
            }

            self.handle_first_effect();
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

        if let Err(effect) = self.evaluator.step(instructions) {
            self.effects.add_effect(effect);
        }

        self.most_recent_step = Some(next_instruction);

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&next_instruction)
        {
            self.effects.add_effect(Effect::Breakpoint);
        }
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Effects {
    unhandled_effects: VecDeque<Effect>,
}

impl Effects {
    pub fn first_unhandled_effect(&self) -> Option<&Effect> {
        self.unhandled_effects.front()
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.unhandled_effects.push_back(effect);
    }
}
