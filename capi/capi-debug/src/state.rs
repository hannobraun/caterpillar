use capi_runtime::{Function, Program, ProgramState};
use leptos::{ReadSignal, SignalGet};

pub struct ExecutionContext {
    pub function: Function,
    pub message: Option<&'static str>,
}

impl ExecutionContext {
    pub fn from_program(
        program: ReadSignal<Option<Program>>,
    ) -> Result<Self, &'static str> {
        let Some(program) = program.get() else {
            return Err("No program available.");
        };

        let (_effect, address) = match &program.state {
            ProgramState::Running => {
                return Err("Program is running.");
            }
            ProgramState::Finished => {
                return Err("Program has finished running.");
            }
            ProgramState::Effect { effect, address } => (effect, address),
        };

        let Some(location) = program.source_map.address_to_location(address)
        else {
            return Err(
                "Program is stopped at instruction with no associated source \
                location.",
            );
        };

        let function = program
            .functions
            .inner
            .iter()
            .find(|function| function.name == location.function())
            .cloned();
        let Some(function) = function else {
            return Err(
                "Program stopped at unknown function. This is most likely a \
                bug in Caterpillar.",
            );
        };

        Ok(Self {
            function,
            message: None,
        })
    }
}
