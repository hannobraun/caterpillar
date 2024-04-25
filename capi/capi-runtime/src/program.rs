use crate::{
    builtins, evaluator::EvaluatorState, source_map::SourceMap, Evaluator,
    Functions, LineLocation,
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

    /// Get `LineLocation` for current instruction
    ///
    /// This might return `None`, as not all instructions have locations in the
    /// code. Return instructions are an example of that.
    ///
    /// This shouldn't matter, since users can't set breakpoints there, nor do
    /// those instructions produce errors. So in cases where you actually need a
    /// location, this should return one.
    pub fn location(&self) -> Option<LineLocation> {
        self.source_map
            .inner
            .get(&self.evaluator.next_instruction)
            .cloned()
    }

    fn step_inner(&mut self, mem: &mut [u8]) -> ProgramState {
        if let Some(location) = self.breakpoint_set_for_next_instruction() {
            return ProgramState::Paused { location };
        }

        self.evaluator.step(mem).into()
    }

    fn breakpoint_set_for_next_instruction(&self) -> Option<LineLocation> {
        let location = self.location()?;

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
            return Some(location);
        }

        None
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

    Error(builtins::Error),
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
            EvaluatorState::Error(err) => Self::Error(err),
        }
    }
}
