use serde_big_array::BigArray;

use crate::{
    breakpoints::Breakpoints,
    evaluator::{EvaluatorEffect, EvaluatorState},
    source_map::SourceMap,
    DataStack, Evaluator, Functions, InstructionAddress, Value,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub breakpoints: Breakpoints,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry_address: InstructionAddress,

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
    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry_address);
        self.state = ProgramState::default();
        self.previous_data_stack.clear();
        self.memory.zero();
    }

    pub fn push(&mut self, arguments: impl IntoIterator<Item = Value>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn step(&mut self) {
        if !self.state.is_running() {
            return;
        }

        self.state = self.step_inner();
    }

    pub fn step_inner(&mut self) -> ProgramState {
        // This method is separate from the main `step` method, so we can just
        // return `ProgramState`s here, and have `step` take care of saving them
        // in `self.state` automatically.

        self.previous_data_stack = self.evaluator.data_stack.clone();
        let evaluator_state = self.evaluator.step();

        if let EvaluatorState::Running { just_executed } = evaluator_state {
            // We only ever want to pause the program due to a breakpoint, if
            // the evaluator is running normally. Else, we might mask errors or
            // other important states.

            if self
                .breakpoints
                .should_stop_at_and_clear_ephemeral(&just_executed)
            {
                return ProgramState::Effect {
                    effect: ProgramEffect {
                        kind: ProgramEffectKind::Paused,
                        address: just_executed,
                    },
                };
            }
        }

        match evaluator_state {
            EvaluatorState::Running { .. } => ProgramState::Running,
            EvaluatorState::Finished => ProgramState::Finished,
            EvaluatorState::Effect { effect, address } => {
                ProgramState::Effect {
                    effect: ProgramEffect {
                        kind: ProgramEffectKind::Evaluator(effect),
                        address,
                    },
                }
            }
        }
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum ProgramState {
    #[default]
    Running,

    Finished,

    Effect {
        effect: ProgramEffect,
    },
}

impl ProgramState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProgramEffect {
    pub kind: ProgramEffectKind,
    pub address: InstructionAddress,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProgramEffectKind {
    Evaluator(EvaluatorEffect),
    Paused,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Memory {
    #[serde(with = "BigArray")]
    pub inner: [u8; 256],
}

impl Memory {
    pub fn zero(&mut self) {
        *self = Self::default();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self { inner: [0; 256] }
    }
}
