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
        is_in_innermost_active_function: bool,
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
                    is_in_innermost_active_function,
                    fragments,
                    source_map,
                    breakpoints,
                    effects,
                )
            })
            .collect::<Vec<_>>();

        Self { parameters, body }
    }
}
