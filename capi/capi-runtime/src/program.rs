use std::collections::BTreeMap;

use crate::{
    builtins::Effect, evaluator::EvaluatorState, source_map::SourceMap,
    DataStack, Evaluator, Functions, InstructionAddress, Value,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub breakpoints: BTreeMap<InstructionAddress, bool>,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry_address: InstructionAddress,

    /// The most recently executed instruction
    pub current_instruction: InstructionAddress,

    /// The data stack, before the most recent instruction was executed
    pub previous_data_stack: DataStack,

    /// Indicate whether the program was halted
    pub halted: bool,
}

impl Program {
    pub fn reset(&mut self) {
        self.evaluator.reset(self.entry_address);
        self.state = ProgramState::default();
        self.current_instruction = InstructionAddress::default();
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

    pub fn toggle_breakpoint(&mut self, address: InstructionAddress) {
        let breakpoint = self.breakpoints.entry(address).or_insert(false);
        *breakpoint = !*breakpoint;
    }

    pub fn breakpoint_at(&self, address: &InstructionAddress) -> bool {
        self.breakpoints.get(address) == Some(&true)
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

        if self.halted {
            return ProgramState::Effect {
                effect: ProgramEffect::Halted,
                address: self.current_instruction,
            };
        }

        let address = self.current_instruction;
        if self.breakpoint_at(&address) {
            return ProgramState::Effect {
                effect: ProgramEffect::Paused,
                address,
            };
        }

        self.previous_data_stack = self.evaluator.data_stack.clone();
        self.current_instruction = self.evaluator.next_instruction;

        self.evaluator.step().into()
    }

    pub fn halt(&mut self) {
        self.halted = true;
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
        address: InstructionAddress,
    },
}

impl ProgramState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}

impl From<EvaluatorState> for ProgramState {
    fn from(state: EvaluatorState) -> Self {
        match state {
            EvaluatorState::Running => Self::Running,
            EvaluatorState::Finished => Self::Finished,
            EvaluatorState::Effect { effect, address } => Self::Effect {
                effect: ProgramEffect::Builtin(effect),
                address,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProgramEffect {
    Builtin(Effect),
    Halted,
    Paused,
}
