use std::collections::VecDeque;

use crate::{
    breakpoints::Breakpoints,
    evaluator::{evaluate, EvaluatorState},
    runtime::{self, Value},
    BuiltinEffect, Code, EvaluatorEffect, Stack,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Process {
    state: ProcessState,
    stack: Stack,
    breakpoints: Breakpoints,
}

impl Process {
    pub fn state(&self) -> &ProcessState {
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

    pub fn reset(&mut self, entry: runtime::Function, arguments: Vec<Value>) {
        self.state = ProcessState::default();
        self.stack = Stack::default();

        self.stack
            .push_frame(entry)
            .expect("Expected recursion limit to be more than zero.");
        self.push(arguments);
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        for value in values {
            self.stack.push_operand(value);
        }
    }

    pub fn clear_durable_breakpoint(&mut self, location: runtime::Location) {
        self.breakpoints.clear_durable(location);
    }

    pub fn set_durable_breakpoint(&mut self, location: runtime::Location) {
        self.breakpoints.set_durable(location);
    }

    pub fn continue_(&mut self, and_stop_at: Option<runtime::Location>) {
        if let Some(EvaluatorEffect::Builtin(BuiltinEffect::Breakpoint)) =
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

    pub fn step(&mut self, code: &Code) {
        if !self.state.can_step() {
            return;
        }

        let next_instruction = self.stack.next_instruction_overall().unwrap();
        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(next_instruction.clone())
        {
            self.state.add_effect(EvaluatorEffect::Builtin(
                BuiltinEffect::Breakpoint,
            ));
        }

        match evaluate(code, &mut self.stack) {
            Ok(EvaluatorState::Running) => {
                self.state.most_recent_step = Some(next_instruction);
            }
            Ok(EvaluatorState::Finished) => {
                self.state.has_finished = true;
            }
            Err(effect) => {
                self.state.add_effect(effect);
            }
        };
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct ProcessState {
    most_recent_step: Option<runtime::Location>,
    unhandled_effects: VecDeque<EvaluatorEffect>,
    has_finished: bool,
}

impl ProcessState {
    pub fn most_recent_step(&self) -> Option<runtime::Location> {
        self.most_recent_step.clone()
    }

    pub fn first_unhandled_effect(&self) -> Option<&EvaluatorEffect> {
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

    pub fn add_effect(&mut self, effect: EvaluatorEffect) {
        self.unhandled_effects.push_back(effect);
    }
}
