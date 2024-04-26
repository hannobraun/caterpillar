use crate::{
    builtins, evaluator::EvaluatorState, source_map::SourceMap, Evaluator,
    Functions, InstructionAddress, LineLocation,
};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Program {
    pub functions: Functions,
    pub source_map: SourceMap,
    pub evaluator: Evaluator,
    pub state: ProgramState,
    pub entry_address: InstructionAddress,
}

impl Program {
    pub fn push(&mut self, arguments: impl IntoIterator<Item = usize>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.next_instruction = self.entry_address;
    }

    pub fn step(&mut self, mem: &mut [u8]) -> ProgramState {
        let state = self.step_inner(mem);
        self.state = state.clone();
        state
    }

    /// Get `LineLocation` for the provided instruction
    ///
    /// This might return `None`, as not all instructions have locations in the
    /// code. Return instructions are an example of that.
    ///
    /// This shouldn't matter, since users can't set breakpoints there, nor do
    /// those instructions produce errors, nor should they show up in call
    /// stacks. So in cases where you actually need a location, this should
    /// return one.
    pub fn location(
        &self,
        instruction: InstructionAddress,
    ) -> Option<LineLocation> {
        self.source_map
            .address_to_location
            .get(&instruction.0)
            .cloned()
    }

    fn step_inner(&mut self, mem: &mut [u8]) -> ProgramState {
        if let Some(location) = self.breakpoint_set_for_next_instruction() {
            return ProgramState::Paused { location };
        }

        self.evaluator.step(mem).into()
    }

    fn breakpoint_set_for_next_instruction(&self) -> Option<LineLocation> {
        let next_location = self.location(self.evaluator.next_instruction)?;

        let function = self
            .functions
            .inner
            .iter()
            .find(|function| function.name == next_location.function)
            .unwrap();
        let expression = function
            .syntax
            .iter()
            .find(|expression| expression.location == next_location)
            .unwrap();

        if expression.breakpoint {
            return Some(next_location);
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
        /// The location at which the program is paused
        location: LineLocation,
    },

    #[default]
    Finished,

    Error {
        err: builtins::Error,
        instruction: InstructionAddress,
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
            EvaluatorState::Error { err, instruction } => {
                Self::Error { err, instruction }
            }
        }
    }
}
