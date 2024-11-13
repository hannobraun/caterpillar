use anyhow::anyhow;
use capi_compiler::{
    code::{
        self, BranchLocation, Cluster, ExpressionLocation, FunctionLocation,
        StableFunctions, Types,
    },
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        function: code::Function,
        name: Option<String>,
        location: FunctionLocation,
        active_expression: Option<&ExpressionLocation>,
        is_innermost_active_function: bool,
        cluster: &Cluster,
        functions: &StableFunctions,
        types: &Types,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
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
                    active_expression,
                    is_innermost_active_function,
                    cluster,
                    functions,
                    types,
                    source_map,
                    breakpoints,
                    effect,
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
