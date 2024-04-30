use std::collections::BTreeMap;

use crate::{
    builtins, evaluator::EvaluatorState, source_map::SourceMap, DataStack,
    DebugEvent, Evaluator, Functions, InstructionAddress, Value,
};

#[derive(
    Clone, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub breakpoints: BTreeMap<InstructionAddress, bool>,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry_address: InstructionAddress,

    /// The most recently executed instruction
    pub most_recent_instruction: InstructionAddress,

    /// The data stack, before the most recent instruction was executed
    pub previous_data_stack: DataStack,

    /// Indicate whether the program was halted
    pub halted: bool,
}

impl Program {
    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry_address);
        self.state = ProgramState::default();
        self.most_recent_instruction = InstructionAddress::default();
        self.previous_data_stack.clear();
        self.halted = false;
    }

    pub fn push(&mut self, arguments: impl IntoIterator<Item = Value>) {
        assert!(
            self.evaluator.data_stack.is_empty(),
            "Pushed arguments to active stack."
        );

        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn apply_debug_event(&mut self, event: DebugEvent) {
        match event {
            DebugEvent::Reset => {
                self.reset();
            }
            DebugEvent::ToggleBreakpoint { address } => {
                let breakpoint =
                    self.breakpoints.entry(address).or_insert(false);
                *breakpoint = !*breakpoint;
            }
        }
    }

    pub fn breakpoint_at(&self, address: &InstructionAddress) -> bool {
        self.breakpoints.get(address) == Some(&true)
    }

    pub fn step(&mut self, mem: &mut [u8]) -> ProgramState {
        if let ProgramState::Error { .. } = self.state {
            return self.state.clone();
        }

        self.state = self.step_inner(mem);
        self.state.clone()
    }

    pub fn step_inner(&mut self, mem: &mut [u8]) -> ProgramState {
        // This method is separate from the main `step` method, so we can just
        // return `ProgramState`s here, and have `step` take care of saving them
        // in `self.state` automatically.

        if self.halted {
            return ProgramState::Error {
                err: ProgramError::Halted,
                address: self.most_recent_instruction,
            };
        }

        let address = self.most_recent_instruction;
        if self.breakpoint_at(&address) {
            return ProgramState::Paused { address };
        }

        self.previous_data_stack = self.evaluator.data_stack.clone();
        self.most_recent_instruction = self.evaluator.next_instruction;

        self.evaluator.step(mem).into()
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum ProgramState {
    Running,

    Paused {
        address: InstructionAddress,
    },

    #[default]
    Finished,

    Error {
        err: ProgramError,
        address: InstructionAddress,
    },
}

impl ProgramState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused { .. })
    }
}

impl From<EvaluatorState> for ProgramState {
    fn from(state: EvaluatorState) -> Self {
        match state {
            EvaluatorState::Running => Self::Running,
            EvaluatorState::Finished => Self::Finished,
            EvaluatorState::Error { err, address } => Self::Error {
                err: err.into(),
                address,
            },
        }
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
pub enum ProgramError {
    #[error(transparent)]
    Builtins(#[from] builtins::Error),

    #[error("The program was halted")]
    Halted,
}
