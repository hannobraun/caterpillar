use std::collections::VecDeque;

use serde_big_array::BigArray;

use crate::{
    breakpoints::Breakpoints,
    code::Code,
    runtime::{
        self, DataStack, Evaluator, EvaluatorEffect, EvaluatorEffectKind,
        EvaluatorState, Value,
    },
    source_map::SourceMap,
    syntax::{self, Functions},
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub breakpoints: Breakpoints,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry: runtime::Function,
    pub arguments: Vec<Value>,

    /// Effects that have not been handled yet
    pub effects: VecDeque<ProgramEffect>,

    /// The data stack, before the most recent instruction was executed
    pub previous_data_stack: DataStack,

    /// Linear memory
    ///
    /// This is accessed via effects handled by the platform, so logically, it
    /// shouldn't be part of `Program`. However, for the time being, having it
    /// here makes it easy to share this with the debugger.
    pub memory: Memory,
}

impl Program {
    pub fn new(
        functions: syntax::Functions,
        source_map: SourceMap,
        code: Code,
        entry: runtime::Function,
        arguments: Vec<Value>,
    ) -> Self {
        let mut evaluator = Evaluator::new(code, entry.clone());
        evaluator.push(arguments.clone());

        Self {
            functions,
            source_map,
            breakpoints: Breakpoints::default(),
            evaluator,
            state: ProgramState::default(),
            entry,
            arguments,
            effects: VecDeque::default(),
            previous_data_stack: DataStack::default(),
            memory: Memory::default(),
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry.clone());
        self.state = ProgramState::default();
        self.effects.clear();
        self.previous_data_stack.clear();
        self.memory.zero();

        self.push(self.arguments.clone());
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        self.evaluator.push(values);
    }

    pub fn can_step(&self) -> bool {
        self.state.is_running() && self.effects.is_empty()
    }

    pub fn step(&mut self) {
        if !self.can_step() {
            return;
        }

        self.state = self.step_inner();
    }

    pub fn step_inner(&mut self) -> ProgramState {
        // This method is separate from the main `step` method, so we can just
        // return `ProgramState`s here, and have `step` take care of saving them
        // in `self.state` automatically.

        self.previous_data_stack = self.evaluator.data_stack().clone();
        let just_executed = match self.evaluator.step() {
            Ok(EvaluatorState::Running { just_executed }) => just_executed,
            Ok(EvaluatorState::Finished) => return ProgramState::Finished,
            Err(EvaluatorEffect { effect, location }) => {
                self.effects.push_back(ProgramEffect {
                    kind: ProgramEffectKind::Evaluator(effect),
                    location: location.clone(),
                });
                location
            }
        };

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&just_executed)
        {
            self.effects.push_back(ProgramEffect {
                kind: ProgramEffectKind::Paused,
                location: just_executed,
            });
        }

        ProgramState::Running
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum ProgramState {
    #[default]
    Running,

    Finished,
}

impl ProgramState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProgramEffect {
    pub kind: ProgramEffectKind,
    pub location: runtime::Location,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProgramEffectKind {
    Evaluator(EvaluatorEffectKind),
    Paused,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Memory {
    #[serde(with = "BigArray")]
    pub inner: [Value; 256],
}

impl Memory {
    pub fn zero(&mut self) {
        *self = Self::default();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            inner: [Value(0); 256],
        }
    }
}
