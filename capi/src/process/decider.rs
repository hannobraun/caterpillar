use crate::{
    breakpoints::Breakpoints,
    runtime::{
        self, DataStack, Evaluator, EvaluatorEffect, EvaluatorState, Value,
    },
};

use super::{Event, State};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    state: State,
    events: Vec<Event>,

    pub evaluator: Evaluator,
    pub entry: runtime::Function,
    pub arguments: Vec<Value>,
}

impl Process {
    pub fn new(
        code: runtime::Code,
        entry: runtime::Function,
        arguments: Vec<Value>,
    ) -> Self {
        let evaluator = Evaluator::new(code, entry.clone());

        let mut self_ = Self {
            state: State::default(),
            events: Vec::new(),
            evaluator,
            entry,
            arguments: arguments.clone(),
        };

        self_.push(arguments);

        self_
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn stack(&self) -> &runtime::Stack {
        self.evaluator.stack()
    }

    pub fn next_instruction(&self) -> Option<runtime::Location> {
        self.evaluator.next_instruction()
    }

    pub fn data_stack(&self) -> &DataStack {
        self.evaluator.data_stack()
    }

    pub fn handle_first_effect(&mut self) {
        self.emit_event(Event::EffectHandled);
    }

    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry.clone());
        self.state = State::default();

        self.push(self.arguments.clone());
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        self.evaluator.push(values);
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

        match self.evaluator.step() {
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
