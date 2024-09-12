use capi_compiler::{
    fragments::{self, FragmentId, Fragments},
    source_map::SourceMap,
};
use capi_process::{Breakpoints, Effect, Process};

use super::Branch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFunction {
    pub name: Option<String>,
    pub branches: Vec<Branch>,
}

impl DebugFunction {
    pub fn new(
        function: fragments::Function,
        active_fragment: Option<FragmentId>,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        process: &Process,
    ) -> Self {
        let effects: Vec<Effect> = process.effects().queue().collect();

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
                    breakpoints,
                    &effects,
                    process,
                )
            })
            .collect();

        Self { name, branches }
    }
}
