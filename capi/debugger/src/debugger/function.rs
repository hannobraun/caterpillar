use capi_compiler::{
    fragments::{self, Fragments},
    source_map::SourceMap,
};
use capi_process::Process;

use super::Branch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub branches: Vec<Branch>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Self {
        let active_fragment = function.branches.iter().find_map(|branch| {
            fragments.inner.iter_from(branch.start).cloned().find_map(
                |fragment| {
                    let instructions =
                        source_map.fragment_to_instructions(&fragment.id());

                    let is_active = if let Some(instructions) = instructions {
                        instructions.iter().copied().any(|mut instruction| {
                            instruction.increment();

                            process
                                .evaluator()
                                .active_instructions()
                                .any(|next| next == instruction)
                        })
                    } else {
                        false
                    };

                    if is_active {
                        Some(fragment.id())
                    } else {
                        None
                    }
                },
            )
        });

        let name = function.name;
        let branches = function
            .branches
            .into_iter()
            .map(|branch| {
                Branch::new(
                    branch,
                    active_fragment,
                    fragments,
                    source_map,
                    process,
                )
            })
            .collect();

        Self { name, branches }
    }
}
