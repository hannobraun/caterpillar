use crate::{evaluator::EvaluatorState, Evaluator, Functions};

#[derive(Clone, Default)]
pub struct Program {
    pub functions: Functions,
    pub evaluator: Evaluator,
    pub entry: usize,
}

impl Program {
    pub fn push(&mut self, arguments: impl IntoIterator<Item = usize>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.instruction = self.entry;
    }

    pub fn step(&mut self, mem: &mut [u8]) -> ProgramState {
        self.evaluator.step(mem).into()
    }
}

pub enum ProgramState {
    Running,
    Finished,
}

impl ProgramState {
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}

impl From<EvaluatorState> for ProgramState {
    fn from(state: EvaluatorState) -> Self {
        match state {
            EvaluatorState::Running => Self::Running,
            EvaluatorState::Finished => Self::Finished,
        }
    }
}
