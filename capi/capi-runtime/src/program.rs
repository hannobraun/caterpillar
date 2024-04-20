use crate::{
    evaluator::EvaluatorState, source_map::SourceMap, Evaluator, Functions,
    LineLocation,
};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry: usize,
}

impl Program {
    pub fn push(&mut self, arguments: impl IntoIterator<Item = usize>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.next_instruction = self.entry;
    }

    pub fn step(&mut self, mem: &mut [u8]) -> ProgramState {
        let state = self.step_inner(mem);
        self.state = state.clone();
        state
    }

    fn step_inner(&mut self, mem: &mut [u8]) -> ProgramState {
        if let Some(location) = self
            .source_map
            .inner
            .get(&self.evaluator.next_instruction)
            .cloned()
        {
            // Not all instructions have a location in the source. Return
            // instructions, for example, don't. That doesn't matter, because
            // The debugger won't show those, and the user won't expect to set
            // breakpoints for them.

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

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum ProgramState {
    Running,

    Paused {
        location: LineLocation,
    },

    #[default]
    Finished,
}

impl ProgramState {
    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused { .. })
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
