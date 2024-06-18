use std::collections::VecDeque;

use crate::{
    breakpoints::Breakpoints,
    runtime::{
        self, DataStack, Evaluator, EvaluatorEffect, EvaluatorEffectKind,
        EvaluatorState, Value,
    },
};

use super::ProcessState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    pub evaluator: Evaluator,
    pub state: ProcessState,
    pub entry: runtime::Function,
    pub arguments: Vec<Value>,

    /// Effects that have not been handled yet
    pub effects: VecDeque<EvaluatorEffect>,

    /// The data stack, before the most recent instruction was executed
    pub previous_data_stack: DataStack,
}

impl Process {
    pub fn new(
        code: runtime::Code,
        entry: runtime::Function,
        arguments: Vec<Value>,
    ) -> Self {
        let mut evaluator = Evaluator::new(code, entry.clone());
        evaluator.push(arguments.clone());

        Self {
            evaluator,
            state: ProcessState::default(),
            entry,
            arguments,
            effects: VecDeque::default(),
            previous_data_stack: DataStack::default(),
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry.clone());
        self.state = ProcessState::default();
        self.effects.clear();
        self.previous_data_stack.clear();

        self.push(self.arguments.clone());
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        self.evaluator.push(values);
    }

    pub fn can_step(&self) -> bool {
        self.state.is_running() && self.effects.is_empty()
    }

    pub fn step(&mut self, breakpoints: &mut Breakpoints) {
        if !self.can_step() {
            return;
        }

        self.state = self.step_inner(breakpoints);
    }

    pub fn step_inner(
        &mut self,
        breakpoints: &mut Breakpoints,
    ) -> ProcessState {
        // This method is separate from the main `step` method, so we can just
        // return `ProcessState`s here, and have `step` take care of saving them
        // in `self.state` automatically.

        let next_instruction = self.evaluator.next_instruction().unwrap();
        if breakpoints
            .should_stop_at_and_clear_ephemeral(next_instruction.clone())
        {
            self.effects.push_back(EvaluatorEffect {
                kind: EvaluatorEffectKind::Builtin(
                    runtime::BuiltinEffect::Breakpoint,
                ),
                location: next_instruction,
            });
        }

        self.previous_data_stack =
            self.evaluator.stack().top_frame().unwrap().data.clone();
        match self.evaluator.step() {
            Ok(EvaluatorState::Running) => {}
            Ok(EvaluatorState::Finished) => return ProcessState::Finished,
            Err(effect) => {
                self.effects.push_back(effect);
            }
        };

        ProcessState::Running
    }
}
