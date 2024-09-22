use anyhow::anyhow;
use capi_compiler::{
    fragments::{self, Fragments, Hash},
    source_map::SourceMap,
};
use capi_runtime::Effect;

use super::{Breakpoints, DebugBranch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFunction {
    pub name: Option<String>,
    pub branches: Vec<DebugBranch>,
}

impl DebugFunction {
    pub fn new(
        function: fragments::Function,
        active_fragment: Option<Hash>,
        is_innermost_active_function: bool,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effects: &[Effect],
    ) -> Self {
        let name = function.name;
        let branches = function
            .branches
            .into_iter()
            .map(|branch| {
                DebugBranch::new(
                    branch,
                    active_fragment,
                    is_innermost_active_function,
                    fragments,
                    source_map,
                    breakpoints,
                    effects,
                )
            })
            .collect();

        Self { name, branches }
    }

    pub fn active_branch(&self) -> anyhow::Result<&DebugBranch> {
        self.branches
            .iter()
            .find(|branch| branch.is_active)
            .ok_or_else(|| {
                anyhow!(
                    "Expected to find active branch in function, but none is."
                )
            })
    }
}
