use anyhow::anyhow;
use capi_compiler::{
    fragments::{
        self, BranchLocation, FragmentLocation, Fragments, FunctionLocation,
    },
    source_map::SourceMap,
    syntax::Cluster,
};
use capi_runtime::Effect;

use super::{Breakpoints, DebugBranch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFunction {
    pub name: Option<String>,
    pub branches: Vec<DebugBranch>,
}

impl DebugFunction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        function: fragments::Function,
        location: FunctionLocation,
        active_fragment: Option<&FragmentLocation>,
        is_innermost_active_function: bool,
        cluster: &Cluster,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effects: &[Effect],
    ) -> Self {
        let name = function.name;
        let branches = function
            .branches
            .into_iter()
            .map(|(index, branch)| {
                DebugBranch::new(
                    branch,
                    BranchLocation {
                        parent: Box::new(location.clone()),
                        index,
                    },
                    active_fragment,
                    is_innermost_active_function,
                    cluster,
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
