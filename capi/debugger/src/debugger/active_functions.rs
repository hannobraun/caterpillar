use std::fmt;

use capi_process::Process;
use capi_protocol::{host::GameEngineHost, updates::SourceCode};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Functions { functions: Vec<Function> },
    Message { message: Message },
}

impl ActiveFunctions {
    pub fn new(
        source_code: Option<&SourceCode>,
        process: Option<&Process<GameEngineHost>>,
    ) -> Self {
        let Some(source_code) = source_code else {
            return Self::Message {
                message: Message::NoServer,
            };
        };
        let Some(process) = process else {
            return Self::Message {
                message: Message::NoProcess,
            };
        };

        if process.state().can_step() {
            return Self::Message {
                message: Message::ProcessRunning,
            };
        }
        if process.state().has_finished() {
            return Self::Message {
                message: Message::ProcessFinished,
            };
        }

        let functions = process
            .stack()
            .all_next_instructions_in_frames()
            .filter_map(|runtime_location| {
                let fragment_id = source_code
                    .source_map
                    .instruction_to_fragment(&runtime_location);
                let function = source_code
                    .fragments
                    .find_function(&fragment_id)
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
pub enum Message {
    NoServer,
    NoProcess,
    ProcessRunning,
    ProcessFinished,
}

impl fmt::Display for Message {
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
