use std::{collections::VecDeque, fmt};

use capi_process::{InstructionAddress, Process};
use capi_protocol::updates::Code;

use super::Branch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Functions { functions: Vec<Branch> },
    Message { message: ActiveFunctionsMessage },
}

impl ActiveFunctions {
    pub fn new(code: Option<&Code>, process: Option<&Process>) -> Self {
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

        if process.can_step() {
            return Self::Message {
                message: ActiveFunctionsMessage::ProcessRunning,
            };
        }
        if process.has_finished() {
            return Self::Message {
                message: ActiveFunctionsMessage::ProcessFinished,
            };
        }

        let mut call_stack: VecDeque<InstructionAddress> =
            process.evaluator().active_instructions().collect();

        let mut functions_and_branches = VecDeque::new();

        while let Some(instruction) = call_stack.pop_front() {
            let Some(fragment_id) =
                code.source_map.instruction_to_fragment(&instruction)
            else {
                panic!(
                    "Expecting all instructions referenced on call stack to \
                    map to a fragment, but instruction `{instruction}` does \
                    not."
                );
            };
            let function_and_branch = code
                .fragments
                .find_function_by_fragment_in_body(&fragment_id)
                .map(|(function, branch)| (function.clone(), branch.clone()));

            if let Some(function_and_branch) = function_and_branch {
                functions_and_branches.push_front(function_and_branch);
            }
        }

        Self::Functions {
            functions: functions_and_branches
                .into_iter()
                .map(|(function, branch)| {
                    Branch::new(
                        function,
                        branch,
                        &code.fragments,
                        &code.source_map,
                        process,
                    )
                })
                .collect(),
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
