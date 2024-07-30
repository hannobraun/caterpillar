use std::fmt;

use capi_process::{InstructionAddr, Process};
use capi_protocol::{host::GameEngineHost, updates::Code};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Functions { functions: Vec<Function> },
    Message { message: ActiveFunctionsMessage },
}

impl ActiveFunctions {
    pub fn new(
        code: Option<&Code>,
        process: Option<&Process<GameEngineHost>>,
    ) -> Self {
        let Some(code) = code else {
            return Self::Message {
                message: ActiveFunctionsMessage::NoServer,
            };
        };
        let Some(process) = process else {
            return Self::Message {
                message: ActiveFunctionsMessage::NoProcess,
            };
        };

        if process.state().can_step() {
            return Self::Message {
                message: ActiveFunctionsMessage::ProcessRunning,
            };
        }
        if process.state().has_finished() {
            return Self::Message {
                message: ActiveFunctionsMessage::ProcessFinished,
            };
        }

        let call_stack: Vec<InstructionAddr> = process
            .stack()
            .all_next_instructions_in_frames()
            .rev()
            .collect();
        let mut functions = Vec::new();

        for instruction in call_stack {
            let fragment_id =
                code.source_map.instruction_to_fragment(&instruction);
            let Some(function) = code
                .fragments
                .find_function_by_fragment(&fragment_id)
                .cloned()
            else {
                unreachable!(
                    "Expecting function referenced from call stack to exist."
                );
            };

            let next_function = Function::new(
                function,
                &code.fragments,
                &code.source_map,
                process,
            );

            functions.push(next_function);
        }

        Self::Functions { functions }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctionsMessage {
    NoServer,
    NoProcess,
    ProcessRunning,
    ProcessFinished,
}

impl fmt::Display for ActiveFunctionsMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoServer => {
                write!(f, "No connection to server.")?;
            }
            Self::NoProcess => {
                write!(f, "No connection to process.")?;
            }
            Self::ProcessRunning => {
                write!(f, "Process is running.")?;
            }
            Self::ProcessFinished => {
                write!(f, "Process is finished.")?;
            }
        }

        Ok(())
    }
}
