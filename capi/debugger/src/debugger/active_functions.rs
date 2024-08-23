use std::{collections::VecDeque, fmt};

use capi_compiler::fragments::{self, FragmentId, FragmentKind, Payload};
use capi_process::{InstructionAddress, Process};
use capi_protocol::updates::Code;

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Entries { entries: Vec<ActiveFunctionsEntry> },
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

        let mut entries = VecDeque::new();

        if let Some(outer) = call_stack.front() {
            let (outer, _) = instruction_to_function(outer, code);
            if outer.name.as_deref() != Some("main") {
                let main_id = code
                    .fragments
                    .inner
                    .find_function_by_name("main")
                    .expect("Expecting `main` function to exist.");
                let main_fragment =
                    code.fragments.inner.inner.get(&main_id).expect(
                        "Just got this `FragmentId` by searching for a \
                        function. Must refer to a valid fragment.",
                    );

                let FragmentKind::Payload {
                    payload: Payload::Function { function: main },
                    ..
                } = &main_fragment.kind
                else {
                    panic!(
                        "Got fragment by specifically searching for `main` \
                        function. Expecting it to be a function fragment."
                    );
                };

                let tail_call = if main.branches.len() == 1 {
                    if let Some(branch) = main.branches.first() {
                        let mut tail_call = None;

                        for fragment in
                            code.fragments.inner.iter_from(branch.start)
                        {
                            match fragment.kind {
                                FragmentKind::Terminator => {}
                                _ => tail_call = Some(fragment.id()),
                            }
                        }

                        tail_call
                    } else {
                        None
                    }
                } else {
                    None
                };

                entries.push_front(Function::new(
                    main.clone(),
                    tail_call,
                    &code.fragments,
                    &code.source_map,
                    process,
                ));
            }
        }

        while let Some(instruction) = call_stack.pop_front() {
            let (function, active_fragment) =
                instruction_to_function(&instruction, code);
            entries.push_front(Function::new(
                function,
                Some(active_fragment),
                &code.fragments,
                &code.source_map,
                process,
            ));
        }

        Self::Entries {
            entries: entries
                .into_iter()
                .map(ActiveFunctionsEntry::Function)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctionsEntry {
    Function(Function),
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
) -> (fragments::Function, FragmentId) {
    // All instructions addresses on the call stack point point to the _next_
    // instruction to execute in the respective frame. Let's make sure we get
    // the correct address before translating it into a fragment.
    let instruction = InstructionAddress {
        index: instruction.index - 1,
    };

    let Some(fragment_id) =
        code.source_map.instruction_to_fragment(&instruction)
    else {
        panic!(
            "Expecting all instructions referenced on call stack to map to a \
            fragment, but instruction `{instruction}` does not."
        );
    };

    let (function, _) = code
        .fragments
        .find_function_by_fragment_in_body(&fragment_id)
        .expect(
            "Expecting code that is referenced on call stack to be part of a \
            known function.",
        );

    (function.clone(), fragment_id)
}
