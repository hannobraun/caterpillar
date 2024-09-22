use anyhow::anyhow;
use capi_compiler::{
    fragments::{Branch, Fragment, FragmentId, Fragments, Hash},
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
    pub fn new(
        branch: Branch,
        active_fragment: Option<Hash<Fragment>>,
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
            .filter_map(|(id, fragment)| {
                DebugFragment::new(
                    id,
                    fragment.clone(),
                    active_fragment,
                    is_in_innermost_active_function,
                    fragments,
                    source_map,
                    breakpoints,
                    effects,
                )
            })
            .collect::<Vec<_>>();

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
        id: &FragmentId,
    ) -> anyhow::Result<Option<&DebugFragment>> {
        if !self.body.iter().any(|fragment| fragment.data.id == *id) {
            return Err(anyhow!(
                "Expected fragment to be in branch, but could not find it. \
                Fragment:\n\
                {id:#?}\n\
                Branch:\n\
                {self:#?}"
            ));
        }

        let mut fragments = self
            .body
            .iter()
            .skip_while(|fragment| fragment.data.id != *id);

        // This is the fragment we've been passed as an argument. Need to ignore
        // it, to advance the iterator to the one we're actually looking for.
        assert_eq!(
            fragments.next().map(|fragment| fragment.data.id).as_ref(),
            Some(id)
        );

        Ok(fragments.next())
    }
}
