use std::{collections::VecDeque, fmt};

use capi_compiler::repr::fragments::{
    self, FragmentExpression, FragmentPayload,
};
use capi_process::{InstructionAddress, Process};
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

        let mut call_stack: VecDeque<InstructionAddress> =
            process.stack().all_next_instructions_in_frames().collect();

        if let Some(instruction) = call_stack.front() {
            let function = instruction_to_function(instruction, code);

            if function.name != "main" {
                let main_function = code
                    .fragments
                    .find_function_by_name("main")
                    .expect("Expected `main` function to exist.");
                add_missing_instruction_from_user_function_to_call_stack(
                    &mut call_stack,
                    main_function,
                    code,
                );
            }
        }

        let mut functions = VecDeque::new();
        let mut previous_instruction: Option<InstructionAddress> = None;

        while let Some(instruction) = call_stack.pop_front() {
            let function = instruction_to_function(&instruction, code);

            if let Some(previous_instruction) = previous_instruction {
                let caller_index =
                    previous_instruction.index.checked_sub(1).expect(
                        "Expected current instruction to not have index `0`. \
                        This instruction index is reserved for the pseudo \
                        function that calls the entry function. And that \
                        should have been removed by tail call optimization, \
                        therefore not show up in the call stack.",
                    );
                let caller_address = InstructionAddress {
                    index: caller_index,
                };
                let caller_id =
                    code.source_map.instruction_to_fragment(&caller_address);

                let caller_function = functions.front().expect(
                    "We have a caller, meaning we must have previously added a \
                    function to `functions`.",
                );
                assert_eq!(
                    code.fragments.find_function_by_fragment(&caller_id),
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
                    code.fragments.inner.inner.get(&caller_id).expect(
                        "Expecting fragment referenced from call stack to \
                        exist.",
                    );
                let FragmentPayload::Expression {
                    expression:
                        FragmentExpression::ResolvedUserFunction {
                            name: called_by_caller,
                        },
                    ..
                } = &caller_fragment.payload
                else {
                    unreachable!(
                        "`caller_fragment` specifically is the fragment that \
                        called the function we're currently looking at. Unless \
                        there is a bug in the preceding code, it must thus be \
                        an expression of the type that we expect."
                    );
                };

                if called_by_caller != &function.name {
                    // The most recent caller did not actually call the function
                    // that we are currently looking at. This means we have
                    // detected a gap in the call stack, created by tail call
                    // optimization.
                    //
                    // Before we fix that up, first put back the current
                    // instruction. After the fixup, we're done with this
                    // iteration of the loop, and we need it back in place if we
                    // don't want to use it.
                    call_stack.push_front(instruction);

                    // Getting the called function is easy enough.
                    let called_function = code
                        .fragments
                        .find_function_by_name(called_by_caller)
                        .expect(
                            "We got this function name from an expression that \
                            is specifically a resolved user function. \
                            Expecting it to exist.",
                        );

                    add_missing_instruction_from_user_function_to_call_stack(
                        &mut call_stack,
                        called_function,
                        code,
                    );

                    // We've added the missing stack frame! This might have
                    // closed the gap, or there might be more stack frames
                    // missing. Either way, we'll find out in the next iteration
                    // of the loop.
                    continue;
                }
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

fn instruction_to_function(
    instruction: &InstructionAddress,
    code: &Code,
) -> fragments::Function {
    let fragment_id = code.source_map.instruction_to_fragment(instruction);
    code.fragments
        .find_function_by_fragment(&fragment_id)
        .cloned()
        .expect("Expecting function referenced from call stack to exist.")
}

fn add_missing_instruction_from_user_function_to_call_stack(
    call_stack: &mut VecDeque<InstructionAddress>,
    missing_function: &fragments::Function,
    code: &Code,
) {
    // Figuring out which instruction we need to add to the call stack to
    // replace the missing stack frame is straight-forward: The function was
    // optimized away, so the instruction that called the next function must
    // have been the last one!
    let mut next_id = missing_function.start;
    let terminator = loop {
        let next_fragment = code.fragments.inner.inner.get(&next_id).expect(
            "Fragment is referenced as a next fragment. \
                                Expecting it to exist.",
        );

        match next_fragment.payload {
            FragmentPayload::Expression { next, .. } => {
                next_id = next;
            }
            FragmentPayload::Terminator => {
                break next_id;
            }
        }
    };

    // And now we can fix up the gap.
    let missing_instruction = code
        .source_map
        .fragment_to_instructions(&terminator)
        .expect("Expecting fragment to map to instruction");
    call_stack.push_front(missing_instruction[0]);
}
