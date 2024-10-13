use std::{collections::VecDeque, fmt};

use anyhow::anyhow;
use capi_compiler::fragments::{
    self, FragmentLocation, FunctionIndexInRootContext, FunctionLocation,
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
            let (outer, _) = instruction_to_named_function(outer, code);
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

        while let Some(address) = active_instructions.pop_front() {
            let (named_function, function_index_in_root_context) =
                instruction_to_named_function(&address, code);
            let active_fragment =
                code.source_map.instruction_to_fragment(&address);

            if let Some(expected_name) = &expected_next_function {
                if Some(expected_name) != named_function.name.as_ref() {
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
                active_fragment.and_then(|active_fragment| {
                    function_call_to_function_name(active_fragment, code)
                });

            let location = FunctionLocation::NamedFunction {
                index: function_index_in_root_context,
            };

            let cluster = code
                .fragments
                .find_cluster_by_named_function(&location)
                .expect("All named functions must be part of a cluster.");
            entries.push_front(ActiveFunctionsEntry::Function(
                DebugFunction::new(
                    named_function,
                    location,
                    active_fragment,
                    active_instructions.is_empty(),
                    cluster,
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
        fragment: &FragmentLocation,
    ) -> anyhow::Result<Option<DebugFragment>> {
        if let Some(after) = branch.fragment_after(fragment)? {
            return Ok(Some(after.clone()));
        }

        self.find_next_fragment_after_caller(fragment)
    }

    pub fn find_next_fragment_after_caller(
        &self,
        fragment: &FragmentLocation,
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
            .find(|branch| {
                !branch.body.iter().any(|f| f.data.location == *fragment)
            });

        let Some(caller_branch) = caller_branch else {
            return Ok(None);
        };

        let caller = caller_branch.active_fragment()?;

        self.find_next_fragment_or_next_after_caller(
            caller_branch,
            &caller.data.location,
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

fn instruction_to_named_function(
    address: &InstructionAddress,
    code: &Code,
) -> (fragments::Function, FunctionIndexInRootContext) {
    let location = code.source_map.instruction_to_function(address).expect(
        "Expecting instructions on call stack to all map to a function.",
    );

    let mut current_location = location.clone();

    loop {
        match current_location {
            FunctionLocation::NamedFunction { index } => {
                let function = code
                    .fragments
                    .named_functions
                    .inner
                    .get(&index)
                    .expect(
                        "Function location in source map should refer to \
                        function.",
                    )
                    .clone();

                return (function, index);
            }
            FunctionLocation::AnonymousFunction { location } => {
                current_location = *location.parent.parent;
            }
        }
    }
}

fn reconstruct_function(
    name: &str,
    entries: &mut VecDeque<ActiveFunctionsEntry>,
    code: &Code,
    breakpoints: &Breakpoints,
    effects: &[Effect],
) -> Option<String> {
    let Some(function) =
        code.fragments.named_functions.find_function_by_name(name)
    else {
        panic!("Expecting function `{name}` to exist.");
    };

    let tail_call = if let Some(branch) = function.find_single_branch() {
        let mut tail_call = None;

        for fragment in branch.body() {
            tail_call = Some(fragment.location);
        }

        tail_call
    } else {
        None
    };

    let expected_next_function = tail_call
        .as_ref()
        .and_then(|tail_call| function_call_to_function_name(tail_call, code));

    let cluster = code
        .fragments
        .find_cluster_by_named_function(&function.location)
        .expect("All functions must be part of a cluster.");
    entries.push_front(ActiveFunctionsEntry::Function(DebugFunction::new(
        function.clone(),
        function.location,
        tail_call.as_ref(),
        false,
        cluster,
        &code.fragments,
        &code.source_map,
        breakpoints,
        effects,
    )));

    expected_next_function
}

fn function_call_to_function_name(
    function_call: &FragmentLocation,
    code: &Code,
) -> Option<String> {
    let fragment = code
        .fragments
        .named_functions
        .find_fragment_by_location(function_call)
        .expect("Fragment referenced by active function must exist.");
    let hash = fragment.as_call_to_function()?;
    let function = code.fragments.named_functions.find_by_hash(hash)?;

    function.name.clone()
}
