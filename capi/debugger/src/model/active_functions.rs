use std::{collections::VecDeque, fmt};

use anyhow::anyhow;
use capi_compiler::fragments::{
    self, Fragment, FragmentId, FragmentKind, Hash,
};
use capi_protocol::{host_state::HostState, updates::Code};
use capi_runtime::{Effect, InstructionAddress};

use super::{Breakpoints, DebugBranch, DebugFragment, DebugFunction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctions {
    Entries { entries: ActiveFunctionsEntries },
    Message { message: ActiveFunctionsMessage },
}

impl ActiveFunctions {
    pub fn new(
        code: Option<&Code>,
        breakpoints: &Breakpoints,
        state: Option<&HostState>,
    ) -> Self {
        let Some(code) = code else {
            return Self::Message {
                message: ActiveFunctionsMessage::NoServer,
            };
        };
        let (effects, active_instructions) = match state {
            Some(state) => match state {
                HostState::Running => {
                    return Self::Message {
                        message: ActiveFunctionsMessage::ProcessRunning,
                    };
                }
                HostState::Finished => {
                    return Self::Message {
                        message: ActiveFunctionsMessage::ProcessFinished,
                    };
                }
                HostState::Stopped {
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
                call_fragment_to_function_name(active_fragment.this, code);

            entries.push_front(ActiveFunctionsEntry::Function(
                DebugFunction::new(
                    function,
                    Some(active_fragment.this),
                    active_instructions.is_empty(),
                    &code.fragments,
                    &code.source_map,
                    breakpoints,
                    effects,
                ),
            ));
        }

        Self::Entries {
            entries: ActiveFunctionsEntries {
                inner: entries.into(),
            },
        }
    }

    pub fn entries(&self) -> anyhow::Result<&ActiveFunctionsEntries> {
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
pub struct ActiveFunctionsEntries {
    pub inner: Vec<ActiveFunctionsEntry>,
}

impl ActiveFunctionsEntries {
    pub fn leaf(&self) -> &ActiveFunctionsEntry {
        self.inner.first().expect(
            "Empty active function entries should never get constructed. At \
            the very least, the leaf function should be present. If that is \
            not `main`, the `main` function should be present (possibly \
            reconstructed) too.",
        )
    }

    pub fn find_next_fragment_or_next_after_caller(
        &self,
        branch: &DebugBranch,
        fragment: &Hash<Fragment>,
    ) -> anyhow::Result<Option<DebugFragment>> {
        if let Some(after) = branch.fragment_after(fragment)? {
            return Ok(Some(after.clone()));
        }

        self.find_next_fragment_after_caller(fragment)
    }

    pub fn find_next_fragment_after_caller(
        &self,
        fragment: &Hash<Fragment>,
    ) -> anyhow::Result<Option<DebugFragment>> {
        let caller_branch = self
            .inner
            .iter()
            .filter_map(|entry| match entry {
                ActiveFunctionsEntry::Function(function) => Some(function),
                ActiveFunctionsEntry::Gap => None,
            })
            .filter_map(|function| match function.active_branch() {
                Ok(branch) => Some(branch),
                Err(_) => None,
            })
            .find(|branch| !branch.body.iter().any(|f| f.hash() == *fragment));

        let Some(caller_branch) = caller_branch else {
            return Ok(None);
        };

        let caller = caller_branch.active_fragment()?;

        self.find_next_fragment_or_next_after_caller(
            caller_branch,
            &caller.hash(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveFunctionsEntry {
    Function(DebugFunction),
    Gap,
}

impl ActiveFunctionsEntry {
    pub fn function(&self) -> anyhow::Result<&DebugFunction> {
        let Self::Function(function) = self else {
            return Err(anyhow!(
                "Expected active functions entry to be function. Got \
                instead:\n\
                {self:#?}"
            ));
        };

        Ok(function)
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
        let fragment = code.fragments.get(&fragment_id.this);
        panic!(
            "Active instruction `{instruction}` maps to active fragment \
            `{fragment_id:?}`. Expecting that fragment to be part of the body \
            of a named function, but it isn't.\n\
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

            for (id, fragment) in code.fragments.iter_from(branch.start) {
                match fragment.kind {
                    FragmentKind::Terminator => {}
                    _ => tail_call = Some(id.this),
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
        .and_then(|tail_call| call_fragment_to_function_name(tail_call, code));

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

fn call_fragment_to_function_name(
    call_fragment: Hash<Fragment>,
    code: &Code,
) -> Option<String> {
    let fragment = code
        .fragments
        .get(&call_fragment)
        .expect("Fragment referenced by active function must exist.");

    let FragmentKind::CallToFunction { name, .. } = &fragment.kind else {
        return None;
    };

    Some(name.clone())
}
