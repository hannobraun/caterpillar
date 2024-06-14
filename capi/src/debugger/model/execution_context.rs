use crate::process::{Process, ProcessState};

use super::Function;

#[derive(Clone, Eq, PartialEq)]
pub struct ExecutionContext {
    /// The function of the current execution context
    ///
    /// Can be `None` on initialization, before the program becomes available.
    /// Even if there is no valid execution context right now, for example
    /// because the program is running, the function from the most recent
    /// execution context is available.
    pub function: Option<Function>,

    /// A message that explains why the current execution is not valid
    ///
    /// If this is `Some`, that means that the execution context is not valid.
    pub message: Option<&'static str>,
}

impl ExecutionContext {
    pub fn from_process(prev: Option<&Self>, process: Option<Process>) -> Self {
        let function = prev.and_then(|state: &Self| state.function.clone());

        let Some(process) = process else {
            return Self {
                function,
                message: Some("No program available."),
            };
        };

        let Some(effect) = process.effects.front() else {
            match &process.state {
                ProcessState::Running => {
                    return Self {
                        function,
                        message: Some("Program is running."),
                    };
                }
                ProcessState::Finished => {
                    return Self {
                        function,
                        message: Some("Program has finished running."),
                    };
                }
            };
        };

        let location = process.source_map.runtime_to_syntax(&effect.location);

        let function = process
            .functions
            .get_from_location(location)
            .cloned()
            .map(|function| Function::new(function, &process));
        let Some(function) = function else {
            return Self {
                function,
                message: Some(
                    "Program stopped at unknown function. This is most likely \
                    a bug in Caterpillar.",
                ),
            };
        };

        Self {
            function: Some(function),
            message: None,
        }
    }
}
