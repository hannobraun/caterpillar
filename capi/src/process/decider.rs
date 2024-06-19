use crate::{
    breakpoints::Breakpoints,
    runtime::{
        self, Evaluator, EvaluatorEffect, EvaluatorState, Operands, Stack,
        Value,
    },
};

use super::{Event, State};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    state: State,
    events: Vec<Event>,

    stack: Stack,
    evaluator: Evaluator,
    entry: runtime::Function,
    arguments: Vec<Value>,
}

impl Process {
    pub fn new(
        code: runtime::Code,
        entry: runtime::Function,
        arguments: Vec<Value>,
    ) -> Self {
        let mut self_ = Self {
            state: State::default(),
            events: Vec::new(),
            stack: Stack::new(),
            evaluator: Evaluator::new(code),
            entry: entry.clone(),
            arguments: arguments.clone(),
        };

        self_
            .stack
            .push_frame(entry)
            .expect("Expected recursion limit to be more than zero.");
        self_.push(arguments);

        self_
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn stack(&self) -> &runtime::Stack {
        &self.stack
    }

    pub fn next_instruction(&self) -> Option<runtime::Location> {
        self.stack.next_instruction()
    }

    pub fn operands(&self) -> &Operands {
        self.stack.operands()
    }

    pub fn handle_first_effect(&mut self) {
        self.emit_event(Event::EffectHandled);
    }

    pub fn reset(&mut self) {
        self.state = State::default();
        self.stack = Stack::new();

        self.stack
            .push_frame(self.entry.clone())
            .expect("Expected recursion limit to be more than zero.");
        self.push(self.arguments.clone());
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        for value in values {
            self.stack.push_operand(value);
        }
    }

    pub fn step(&mut self, breakpoints: &mut Breakpoints) {
        if !self.state.can_step() {
            return;
        }

        let next_instruction = self.next_instruction().unwrap();
        if breakpoints
            .should_stop_at_and_clear_ephemeral(next_instruction.clone())
        {
            self.emit_event(Event::EffectTriggered {
                effect: EvaluatorEffect::Builtin(
                    runtime::BuiltinEffect::Breakpoint,
                ),
            });
        }

        match self.evaluator.step(&mut self.stack) {
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

    pub fn take_events(&mut self) -> impl Iterator<Item = Event> + '_ {
        self.events.drain(..)
    }

    fn emit_event(&mut self, event: Event) {
        self.events.push(event.clone());
        self.state.evolve(event);
    }
}
