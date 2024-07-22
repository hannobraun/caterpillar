use std::collections::VecDeque;

use crate::{
    breakpoints::Breakpoints,
    evaluator::{evaluate, EvaluatorState},
    instructions::InstructionAddr,
    Bytecode, CoreEffect, Effect, Host, Stack, Value,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Process<H: Host> {
    state: ProcessState<H>,
    stack: Stack,
    breakpoints: Breakpoints,
}

impl<H: Host> Process<H> {
    pub fn state(&self) -> &ProcessState<H> {
        &self.state
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    pub fn handle_first_effect(&mut self) {
        self.state.unhandled_effects.pop_front();
    }

    pub fn reset(
        &mut self,
        bytecode: &Bytecode,
        arguments: impl IntoIterator<Item = Value>,
    ) {
        self.state = ProcessState::default();
        self.stack = Stack::default();

        self.stack
            .push_frame(bytecode.entry().unwrap(), &bytecode.instructions)
            .expect("Expected recursion limit to be more than zero.");
        self.push(arguments);
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        for value in values {
            self.stack.push_operand(value);
        }
    }

    pub fn clear_durable_breakpoint(&mut self, instruction: &InstructionAddr) {
        self.breakpoints.clear_durable(instruction);
    }

    pub fn set_durable_breakpoint(&mut self, instruction: InstructionAddr) {
        self.breakpoints.set_durable(instruction);
    }

    pub fn continue_(&mut self, and_stop_at: Option<InstructionAddr>) {
        if let Some(Effect::Core(CoreEffect::Breakpoint)) =
            self.state.first_unhandled_effect()
        {
            if let Some(instruction) = and_stop_at {
                self.breakpoints.set_ephemeral(instruction);
            }

            self.handle_first_effect();
        }
    }

    pub fn stop(&mut self) {
        let next_instruction = self.stack().next_instruction_overall().unwrap();
        self.breakpoints.set_ephemeral(next_instruction);
    }

    pub fn step(&mut self, bytecode: &Bytecode) {
        if !self.state.can_step() {
            return;
        }

        let next_instruction = self.stack.next_instruction_overall().unwrap();

        match evaluate::<H>(bytecode, &mut self.stack) {
            Ok(EvaluatorState::Running) => {}
            Ok(EvaluatorState::Finished) => {
                self.state.has_finished = true;
            }
            Err(effect) => {
                self.state.add_effect(effect);
            }
        };

        self.state.most_recent_step = Some(next_instruction);

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&next_instruction)
        {
            self.state.add_effect(Effect::Core(CoreEffect::Breakpoint));
        }
    }
}

impl<H: Host> Default for Process<H> {
    fn default() -> Self {
        Self {
            state: Default::default(),
            stack: Default::default(),
            breakpoints: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProcessState<H: Host> {
    most_recent_step: Option<InstructionAddr>,
    unhandled_effects: VecDeque<Effect<H::Effect>>,
    has_finished: bool,
}

impl<H: Host> ProcessState<H> {
    pub fn most_recent_step(&self) -> Option<InstructionAddr> {
        self.most_recent_step.as_ref().copied()
    }

    pub fn first_unhandled_effect(&self) -> Option<&Effect<H::Effect>> {
        self.unhandled_effects.front()
    }

    pub fn is_running(&self) -> bool {
        !self.has_finished
    }

    pub fn has_finished(&self) -> bool {
        self.has_finished
    }

    pub fn can_step(&self) -> bool {
        self.is_running() && self.unhandled_effects.is_empty()
    }

    pub fn add_effect(&mut self, effect: Effect<H::Effect>) {
        self.unhandled_effects.push_back(effect);
    }
}

impl<H: Host> Default for ProcessState<H> {
    fn default() -> Self {
        Self {
            most_recent_step: Default::default(),
            unhandled_effects: Default::default(),
            has_finished: Default::default(),
        }
    }
}
