use std::collections::VecDeque;

use serde_big_array::BigArray;

use crate::{
    breakpoints::Breakpoints,
    debugger::DebugEvent,
    runtime::{
        self, DataStack, Evaluator, EvaluatorEffect, EvaluatorEffectKind,
        EvaluatorState, Value,
    },
    source_map::SourceMap,
    syntax,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Process {
    pub functions: syntax::Functions,
    pub source_map: SourceMap,
    pub breakpoints: Breakpoints,
    pub evaluator: Evaluator,
    pub state: ProcessState,
    pub entry: runtime::Function,
    pub arguments: Vec<Value>,

    /// Effects that have not been handled yet
    pub effects: VecDeque<ProcessEffect>,

    /// The data stack, before the most recent instruction was executed
    pub previous_data_stack: DataStack,

    /// Linear memory
    ///
    /// This is accessed via effects handled by the host, so logically, it
    /// shouldn't be part of `Process`. However, for the time being, having it
    /// here makes it easy to share this with the debugger.
    pub memory: Memory,
}

impl Process {
    pub fn new(
        functions: syntax::Functions,
        source_map: SourceMap,
        code: runtime::Code,
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
            state: ProcessState::default(),
            entry,
            arguments,
            effects: VecDeque::default(),
            previous_data_stack: DataStack::default(),
            memory: Memory::default(),
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry.clone());
        self.state = ProcessState::default();
        self.effects.clear();
        self.previous_data_stack.clear();
        self.memory.zero();

        self.push(self.arguments.clone());
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        self.evaluator.push(values);
    }

    pub fn process_event(&mut self, event: DebugEvent) {
        match event {
            DebugEvent::Continue { and_stop_at } => {
                if let Some(ProcessEffect {
                    kind: ProcessEffectKind::Paused,
                    ..
                }) = self.effects.front()
                {
                    if let Some(instruction) = and_stop_at {
                        self.breakpoints.set_ephemeral(instruction);
                    }

                    self.effects.pop_front();
                }
            }
            DebugEvent::Reset => {
                self.reset();
            }
            DebugEvent::Step => {
                if let Some(ProcessEffect {
                    kind: ProcessEffectKind::Paused,
                    ..
                }) = self.effects.front()
                {
                    self.breakpoints
                        .set_ephemeral(self.evaluator.next_instruction());
                    self.effects.pop_front();
                }
            }
            DebugEvent::Stop => {
                self.breakpoints
                    .set_ephemeral(self.evaluator.next_instruction());
            }
            DebugEvent::ToggleBreakpoint { location } => {
                self.breakpoints.toggle_durable_at(location);
            }
        }
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

    pub fn step_inner(&mut self) -> ProcessState {
        // This method is separate from the main `step` method, so we can just
        // return `ProcessState`s here, and have `step` take care of saving them
        // in `self.state` automatically.

        self.previous_data_stack =
            self.evaluator.stack().top_frame().unwrap().data.clone();
        let just_executed = match self.evaluator.step() {
            Ok(EvaluatorState::Running { just_executed }) => just_executed,
            Ok(EvaluatorState::Finished) => return ProcessState::Finished,
            Err(EvaluatorEffect { effect, location }) => {
                self.effects.push_back(ProcessEffect {
                    kind: ProcessEffectKind::Evaluator(effect),
                    location: location.clone(),
                });
                location
            }
        };

        if self
            .breakpoints
            .should_stop_at_and_clear_ephemeral(&just_executed)
        {
            self.effects.push_back(ProcessEffect {
                kind: ProcessEffectKind::Paused,
                location: just_executed,
            });
        }

        ProcessState::Running
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum ProcessState {
    #[default]
    Running,

    Finished,
}

impl ProcessState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProcessEffect {
    pub kind: ProcessEffectKind,
    pub location: runtime::Location,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProcessEffectKind {
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
