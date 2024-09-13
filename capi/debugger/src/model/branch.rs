use capi_compiler::{
    fragments::{Branch, FragmentId, Fragments},
    source_map::SourceMap,
    syntax::Pattern,
};
use capi_process::{Breakpoints, Effect};

use super::DebugFragment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugBranch {
    pub parameters: Vec<String>,
    pub body: Vec<DebugFragment>,
}

impl DebugBranch {
    pub fn new(
        branch: Branch,
        active_fragment: Option<FragmentId>,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effects: &[Effect],
    ) -> Self {
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
                    effects,
                )
            })
            .collect();

        Self { parameters, body }
    }
}
