use std::{collections::VecDeque, fmt};

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

        let mut call_stack: VecDeque<InstructionAddr> =
            process.stack().all_next_instructions_in_frames().collect();
        let mut functions = VecDeque::new();
        let mut previous_instruction = None;

        while let Some(instruction) = call_stack.pop_front() {
            dbg!(previous_instruction);

            let fragment_id =
                code.source_map.instruction_to_fragment(&instruction);
            let function = code
                .fragments
                .find_function_by_fragment(&fragment_id)
                .cloned()
                .expect(
                    "Expecting function referenced from call stack to exist.",
                );

            let function = Function::new(
                function,
                &code.fragments,
                &code.source_map,
                process,
            );

            functions.push_front(function);
            previous_instruction = Some(instruction);
        }

        Self::Functions {
            functions: functions.into(),
        }
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
