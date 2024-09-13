use std::{collections::VecDeque, fmt};

use capi_compiler::fragments::{self, FragmentId, FragmentKind, Payload};
use capi_process::{Breakpoints, Effect, InstructionAddress};
use capi_protocol::{runtime_state::RuntimeState, updates::Code};

use super::DebugFunction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Entries { entries: Vec<ActiveFunctionsEntry> },
    Message { message: ActiveFunctionsMessage },
}

impl ActiveFunctions {
    pub fn new(
        code: Option<&Code>,
        breakpoints: &Breakpoints,
        state: Option<&RuntimeState>,
    ) -> Self {
        let Some(code) = code else {
            return Self::Message {
                message: ActiveFunctionsMessage::NoServer,
            };
        };
        let (effects, active_instructions) = match state {
            Some(state) => match state {
                RuntimeState::Running => {
                    return Self::Message {
                        message: ActiveFunctionsMessage::ProcessRunning,
                    };
                }
                RuntimeState::Finished => {
                    return Self::Message {
                        message: ActiveFunctionsMessage::ProcessFinished,
                    };
                }
                RuntimeState::Stopped {
                    effects,
                    active_instructions,
                    current_operands: _,
                } => (effects, active_instructions),
            },
            None => {
                return Self::Message {
                    message: ActiveFunctionsMessage::NoProcess,
                };
            }
        };

        let mut active_instructions: VecDeque<InstructionAddress> =
            active_instructions.clone().into();

        let mut entries = VecDeque::new();
        let mut expected_next_function = Some("main".to_string());

        if let Some(outer) = active_instructions.front() {
            let (outer, _) = instruction_to_function(outer, code);
            if outer.name != expected_next_function {
                expected_next_function = reconstruct_function(
                    "main",
                    &mut entries,
                    code,
                    breakpoints,
                    effects,
                );
            }
        }

        while let Some(instruction) = active_instructions.pop_front() {
            let (function, active_fragment) =
                instruction_to_function(&instruction, code);

            if let Some(expected_name) = &expected_next_function {
                if Some(expected_name) != function.name.as_ref() {
                    reconstruct_function(
                        expected_name,
                        &mut entries,
                        code,
                        breakpoints,
                        effects,
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
                    active_instructions.is_empty(),
                    &code.fragments,
                    &code.source_map,
                    breakpoints,
                    effects,
                ),
            ));
        }

        Self::Entries {
            entries: entries.into(),
        }
    }

    #[cfg(test)]
    pub fn entries(&self) -> anyhow::Result<&[ActiveFunctionsEntry]> {
        use anyhow::anyhow;

        let ActiveFunctions::Entries { entries } = &self else {
            return Err(anyhow!(
                "Active function entries not available. Available state:\n\
                {self:#?}"
            ));
        };

        Ok(entries)
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

    let Some((function, _)) = code
        .fragments
        .find_named_function_by_fragment_in_body(&fragment_id)
    else {
        let fragment = code.fragments.get(&fragment_id);
        panic!(
            "Active instruction `{instruction}` maps to active fragment \
            `{fragment_id}`. Expecting that fragment to be part of the body of \
            a named function, but it isn't.\n\
            \n\
            Here's the fragment in question:\n\
            {fragment:#?}"
        );
    };

    (function.clone(), fragment_id)
}

fn reconstruct_function(
    name: &str,
    entries: &mut VecDeque<ActiveFunctionsEntry>,
    code: &Code,
    breakpoints: &Breakpoints,
    effects: &[Effect],
) -> Option<String> {
    let Some(function) = code.fragments.find_function_by_name(name) else {
        panic!("Expecting function `{name}` to exist.");
    };

    let tail_call = if function.branches.len() == 1 {
        if let Some(branch) = function.branches.first() {
            let mut tail_call = None;

            for fragment in code.fragments.iter_from(branch.start) {
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
        false,
        &code.fragments,
        &code.source_map,
        breakpoints,
        effects,
    )));

    expected_next_function
}

fn call_id_to_function_name(id: FragmentId, code: &Code) -> Option<String> {
    let fragment = code
        .fragments
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
