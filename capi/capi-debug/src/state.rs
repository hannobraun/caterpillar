use capi_runtime::{Function, Program, ProgramState};
use leptos::{create_memo, Memo, ReadSignal, SignalGet};

#[derive(Clone, Eq, PartialEq)]
pub struct ExecutionContext {
    pub function: Option<Function>,
    pub message: Option<&'static str>,
}

impl ExecutionContext {
    pub fn from_program(program: ReadSignal<Option<Program>>) -> Memo<Self> {
        create_memo(move |prev| {
            let function = prev.and_then(|state: &Self| state.function.clone());

            let Some(program) = program.get() else {
                return Self {
                    function,
                    message: Some("No program available."),
                };
            };

            let (_effect, address) = match &program.state {
                ProgramState::Running => {
                    return Self {
                        function,
                        message: Some("Program is running."),
                    };
                }
                ProgramState::Finished => {
                    return Self {
                        function,
                        message: Some("Program has finished running."),
                    };
                }
                ProgramState::Effect { effect, address } => (effect, address),
            };

            let Some(location) =
                program.source_map.address_to_location(address)
            else {
                return Self {
                    function,
                    message: Some(
                        "Program is stopped at instruction with no associated \
                    source location.",
                    ),
                };
            };

            let function = program
                .functions
                .inner
                .iter()
                .find(|function| function.name == location.function())
                .cloned();
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
                function: Some(function.clone()),
                message: None,
            }
        })
    }
}
