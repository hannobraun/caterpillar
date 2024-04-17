use crate::{
    evaluator::EvaluatorState, source_map::SourceMap, Evaluator, Functions,
    LineLocation,
};

#[derive(Clone, Default)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
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
        if let Some(location) = self
            .source_map
            .inner
            .get(&self.evaluator.instruction)
            .cloned()
        {
            // Not all instructions have a location in the source. Return
            // instructions, for example, don't.

            let function = self
                .functions
                .inner
                .iter()
                .find(|function| function.name == location.function)
                .unwrap();
            let expression = function
                .syntax
                .iter()
                .find(|expression| expression.location == location)
                .unwrap();

            if expression.breakpoint {
                return ProgramState::Paused { location };
            }
        }

        self.evaluator.step(mem).into()
    }
}

pub enum ProgramState {
    Running,
    Paused { location: LineLocation },
    Finished,
}

impl From<EvaluatorState> for ProgramState {
    fn from(state: EvaluatorState) -> Self {
        match state {
            EvaluatorState::Running => Self::Running,
            EvaluatorState::Finished => Self::Finished,
        }
    }
}
