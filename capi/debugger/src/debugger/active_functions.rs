use std::{collections::VecDeque, fmt};

use capi_compiler::fragments::{self, FragmentId, FragmentKind, Payload};
use capi_process::{InstructionAddress, Process};
use capi_protocol::updates::Code;

use super::DebugFunction;

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
        let mut expected_next_function = Some("main".to_string());

        if let Some(outer) = call_stack.front() {
            let (outer, _) = instruction_to_function(outer, code);
            if outer.name != expected_next_function {
                expected_next_function =
                    reconstruct_function("main", &mut entries, code, process);
            }
        }

        while let Some(instruction) = call_stack.pop_front() {
            let (function, active_fragment) =
                instruction_to_function(&instruction, code);

            if let Some(expected_name) = &expected_next_function {
                if Some(expected_name) != function.name.as_ref() {
                    reconstruct_function(
                        expected_name,
                        &mut entries,
                        code,
                        process,
                    );
                }
            } else {
                entries.push_front(ActiveFunctionsEntry::Gap);
            }

            expected_next_function =
                call_id_to_function_name(active_fragment, code);

            entries.push_front(ActiveFunctionsEntry::Function(
                DebugFunction::new(
                    function,
                    Some(active_fragment),
                    &code.fragments,
                    &code.source_map,
                    process,
                ),
            ));
        }

        Self::Entries {
            entries: entries.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctionsEntry {
    Function(DebugFunction),
    Gap,
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
    let Some(fragment_id) =
        code.source_map.instruction_to_fragment(instruction)
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

fn reconstruct_function(
    name: &str,
    entries: &mut VecDeque<ActiveFunctionsEntry>,
    code: &Code,
    process: &Process,
) -> Option<String> {
    let Some(function_id) = code.fragments.inner.find_function_by_name(name)
    else {
        panic!("Expecting function `{name}` to exist.");
    };
    let function_fragment = code.fragments.inner.inner.get(&function_id).expect(
        "Just got this `FragmentId` by searching for a function. Must refer to a valid fragment.",
    );

    let FragmentKind::Payload {
        payload: Payload::Function { function },
        ..
    } = &function_fragment.kind
    else {
        panic!(
            "Got fragment by specifically searching for a function (`{name}`). \
            Expecting it to be a function fragment."
        );
    };

    let tail_call = if function.branches.len() == 1 {
        if let Some(branch) = function.branches.first() {
            let mut tail_call = None;

            for fragment in code.fragments.inner.iter_from(branch.start) {
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

    let expected_next_function = tail_call
        .and_then(|tail_call| call_id_to_function_name(tail_call, code));

    entries.push_front(ActiveFunctionsEntry::Function(DebugFunction::new(
        function.clone(),
        tail_call,
        &code.fragments,
        &code.source_map,
        process,
    )));

    expected_next_function
}

fn call_id_to_function_name(id: FragmentId, code: &Code) -> Option<String> {
    let fragment = code
        .fragments
        .inner
        .inner
        .get(&id)
        .expect("Fragment referenced by active function must exist.");

    let FragmentKind::Payload {
        payload: Payload::CallToFunction { name, .. },
        ..
    } = &fragment.kind
    else {
        return None;
    };

    Some(name.clone())
}
