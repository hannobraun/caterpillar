use capi_compiler::{
    fragments::{self, FragmentId, Fragments},
    source_map::SourceMap,
    syntax::Pattern,
};
use capi_process::Process;

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch {
    pub parameters: Vec<String>,
    pub body: Vec<Expression>,
}

impl Branch {
    pub fn new(
        branch: fragments::Branch,
        active_fragment: Option<FragmentId>,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
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
            .inner
            .iter_from(branch.start)
            .cloned()
            .filter_map(|fragment| {
                let is_active = Some(fragment.id()) == active_fragment;
                Expression::new(
                    fragment, is_active, fragments, source_map, process,
                )
            })
            .collect();

        Self { parameters, body }
    }
}
