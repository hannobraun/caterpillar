use std::fmt;

use capi_process::{InstructionAddr, Process};
use capi_protocol::{host::GameEngineHost, updates::SourceCode};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Functions { functions: Vec<Function> },
    Message { message: ActiveFunctionsMessage },
}

impl ActiveFunctions {
    pub fn new(
        source_code: Option<&SourceCode>,
        process: Option<&Process<GameEngineHost>>,
    ) -> Self {
        let Some(source_code) = source_code else {
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

        let functions = call_stack
            .into_iter()
            .filter_map(|runtime_location| {
                let fragment_id = source_code
                    .source_map
                    .instruction_to_fragment(&runtime_location);
                let function = source_code
                    .fragments
                    .find_function_by_fragment(&fragment_id)
                    .cloned()?;

                Some(Function::new(
                    function,
                    &source_code.fragments,
                    &source_code.source_map,
                    process,
                ))
            })
            .collect();

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
