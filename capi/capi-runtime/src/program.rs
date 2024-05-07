use std::collections::BTreeMap;

use crate::{
    builtins::Effect, evaluator::EvaluatorState, source_map::SourceMap,
    DataStack, Evaluator, Functions, InstructionAddress, Value,
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

    pub fn breakpoint_at(&self, address: &InstructionAddress) -> bool {
        self.breakpoints.get(address) == Some(&true)
    }

    pub fn step(&mut self) {
        if let ProgramState::Effect { .. } = self.state {
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
                address: self.most_recent_instruction,
            };
        }

        let address = self.most_recent_instruction;
        if self.breakpoint_at(&address) {
            return ProgramState::Paused { address };
        }

        self.previous_data_stack = self.evaluator.data_stack.clone();
        self.most_recent_instruction = self.evaluator.next_instruction;

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
    Running,

    Paused {
        address: InstructionAddress,
    },

    #[default]
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

    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused { .. })
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
}
