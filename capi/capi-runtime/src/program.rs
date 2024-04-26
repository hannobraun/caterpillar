use std::collections::BTreeMap;

use crate::{
    builtins, evaluator::EvaluatorState, source_map::SourceMap, DebugEvent,
    Evaluator, Functions, InstructionAddress, Value,
};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub breakpoints: BTreeMap<InstructionAddress, bool>,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry_address: InstructionAddress,
}

impl Program {
    pub fn push(&mut self, arguments: impl IntoIterator<Item = Value>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.next_instruction = self.entry_address;
    }

    pub fn apply_debug_event(&mut self, event: DebugEvent) {
        match event {
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
        let state = self.step_inner(mem);
        self.state = state.clone();
        state
    }

    fn step_inner(&mut self, mem: &mut [u8]) -> ProgramState {
        let address = self.evaluator.next_instruction;
        if self.breakpoint_at(&address) {
            return ProgramState::Paused { address };
        }

        self.evaluator.step(mem).into()
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
        err: builtins::Error,
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
            EvaluatorState::Error { err, address } => {
                Self::Error { err, address }
            }
        }
    }
}
