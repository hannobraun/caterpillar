use capi_compiler::{
    fragments::{self, FragmentId, Fragments},
    source_map::SourceMap,
    syntax::Pattern,
};
use capi_process::{Breakpoints, Effect, Process};

use super::DebugFragment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch {
    pub parameters: Vec<String>,
    pub body: Vec<DebugFragment>,
}

impl Branch {
    pub fn new(
        branch: fragments::Branch,
        active_fragment: Option<FragmentId>,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        process: &Process,
    ) -> Self {
        let effects: Vec<Effect> = process.effects().queue().collect();

        let parameters = branch
            .parameters
            .inner
            .into_iter()
            .map(|pattern| match pattern {
                Pattern::Identifier { name } => name,
                Pattern::Literal { value } => format!("{value:?}"),
            })
            .collect();
        let body = fragments
            .iter_from(branch.start)
            .cloned()
            .filter_map(|fragment| {
                DebugFragment::new(
                    fragment,
                    active_fragment,
                    fragments,
                    source_map,
                    breakpoints,
                    &effects,
                    process,
                )
            })
            .collect();

        Self { parameters, body }
    }
}
