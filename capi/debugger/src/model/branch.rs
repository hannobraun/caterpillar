use anyhow::anyhow;
use capi_compiler::{
    fragments::{
        Branch, BranchLocation, Cluster, FragmentLocation, NamedFunctions,
    },
    source_map::SourceMap,
    syntax::Pattern,
};
use capi_runtime::Effect;

use super::{Breakpoints, DebugFragment};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugBranch {
    pub parameters: Vec<String>,
    pub body: Vec<DebugFragment>,
    pub is_active: bool,
}

impl DebugBranch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        branch: Branch,
        location: BranchLocation,
        active_fragment: Option<&FragmentLocation>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        named_functions: &NamedFunctions,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effects: &[Effect],
    ) -> Self {
        let body = branch
            .body
            .into_iter()
            .map(|(index, fragment)| {
                let location = FragmentLocation {
                    parent: Box::new(location.clone()),
                    index,
                };
                DebugFragment::new(
                    fragment,
                    location,
                    active_fragment,
                    is_in_innermost_active_function,
                    cluster,
                    named_functions,
                    source_map,
                    breakpoints,
                    effects,
                )
            })
            .collect::<Vec<_>>();
        let parameters = branch
            .parameters
            .inner
            .into_iter()
            .map(|pattern| match pattern {
                Pattern::Identifier { name } => name,
                Pattern::Literal { value } => format!("{value:?}"),
            })
            .collect();

        let is_active =
            body.iter().any(|fragment| fragment.data.state.is_active());

        Self {
            parameters,
            body,
            is_active,
        }
    }

    pub fn active_fragment(&self) -> anyhow::Result<&DebugFragment> {
        self.body
            .iter()
            .find(|fragment| fragment.data.state.is_active())
            .ok_or_else(|| {
                anyhow!(
                    "Expected active fragment in branch, bud could not find \
                    any. Branch:\n\
                    {self:#?}"
                )
            })
    }

    pub fn fragment_after(
        &self,
        fragment: &FragmentLocation,
    ) -> anyhow::Result<Option<&DebugFragment>> {
        if !self.body.iter().any(|f| f.data.location == *fragment) {
            return Err(anyhow!(
                "Expected fragment to be in branch, but could not find it. \
                Fragment:\n\
                {fragment:#?}\n\
                Branch:\n\
                {self:#?}"
            ));
        }

        let mut fragments = self
            .body
            .iter()
            .skip_while(|f| f.data.location != *fragment);

        // This is the fragment we've been passed as an argument. Need to ignore
        // it, to advance the iterator to the one we're actually looking for.
        assert_eq!(
            fragments
                .next()
                .as_ref()
                .map(|fragment| &fragment.data.location),
            Some(fragment)
        );

        Ok(fragments.next())
    }
}
