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
        let mut previous_instruction: Option<InstructionAddr> = None;

        while let Some(instruction) = call_stack.pop_front() {
            let fragment_id =
                code.source_map.instruction_to_fragment(&instruction);
            let function = code
                .fragments
                .find_function_by_fragment(&fragment_id)
                .cloned()
                .expect(
                    "Expecting function referenced from call stack to exist.",
                );

            if let Some(previous_instruction) = previous_instruction {
                let caller_index =
                    previous_instruction.index.checked_sub(1).expect(
                        "Expected current instruction to not have index `0`. \
                        This instruction index is reserved for the pseudo \
                        function that calls the entry function. And that \
                        should have been removed by tail call optimization, \
                        therefore not show up in the call stack.",
                    );
                let caller_address = InstructionAddr {
                    index: caller_index,
                };
                let caller_fragment_id =
                    code.source_map.instruction_to_fragment(&caller_address);

                let caller_function = functions.front().expect(
                    "We have a caller, meaning we must have previously added a \
                    function to `functions`.",
                );
                assert_eq!(
                    code.fragments
                        .find_function_by_fragment(&caller_fragment_id),
                    Some(caller_function),
                    "In theory, subtracting from an instruction address can \
                    land you in a different function than the one you started \
                    with. For this to happen, the original instruction would \
                    have to be the first in its function.\n\
                    \n\
                    The call stack tracks the _next_ instruction to be \
                    executed in each call frame. If that were also the first \
                    instruction, that would mean we're dealing with a totally \
                    fresh stack frame that was just created.\n\
                    \n\
                    Such a stack frame could not have called another function, \
                    since it hasn't done _anything_ yet. And this code \
                    specifically deals with a caller in the stack frame.\n\
                    \n\
                    Therefore, the next instruction could not have been the \
                    first one in the function. And therefore, subtracting from \
                    that address could not have resulted in an address that \
                    points to another function."
                );

                let caller_fragment =
                    code.fragments.inner.inner.get(&caller_fragment_id).expect(
                        "Expecting fragment referenced from call stack to \
                        exist.",
                    );

                dbg!(caller_fragment);
            }

            functions.push_front(function);
            previous_instruction = Some(instruction);
        }

        Self::Functions {
            functions: functions
                .into_iter()
                .map(|function| {
                    Function::new(
                        function,
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
