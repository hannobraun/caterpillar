use crate::{
    breakpoints::Breakpoints,
    runtime::{
        self, evaluate, Code, EvaluatorEffect, EvaluatorState, Stack, Value,
    },
};

use super::{Event, ProcessState};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Process {
    state: ProcessState,
    stack: Stack,
    pub breakpoints: Breakpoints,
}

impl Process {
    pub fn state(&self) -> &ProcessState {
        &self.state
    }

    pub fn stack(&self) -> &runtime::Stack {
        &self.stack
    }

    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    pub fn handle_first_effect(&mut self) {
        self.emit_event(Event::EffectHandled);
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

    pub fn step(&mut self, code: &Code) {
        if !self.state.can_step() {
            return;
        }

        let next_instruction =
            self.stack.state().next_instruction_overall().unwrap();
        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(next_instruction.clone())
        {
            self.emit_event(Event::EffectTriggered {
                effect: EvaluatorEffect::Builtin(
                    runtime::BuiltinEffect::Breakpoint,
                ),
            });
        }

        match evaluate(code, &mut self.stack) {
            Ok(EvaluatorState::Running) => self.emit_event(Event::HasStepped {
                location: next_instruction,
            }),
            Ok(EvaluatorState::Finished) => {
                self.emit_event(Event::Finished);
            }
            Err(effect) => {
                self.emit_event(Event::EffectTriggered { effect });
            }
        };
    }

    fn emit_event(&mut self, event: Event) {
        self.state.evolve(event);
    }
}
