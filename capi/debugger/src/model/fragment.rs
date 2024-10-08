use capi_compiler::{
    fragments::{
        Cluster, Fragment, FragmentId, FragmentLocation, Fragments,
        FunctionLocation,
    },
    host::Host,
    source_map::SourceMap,
};
use capi_game_engine::host::GameEngineHost;
use capi_runtime::Effect;

use super::{Breakpoints, DebugFunction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFragment {
    pub data: DebugFragmentData,
    pub kind: DebugFragmentKind,
}

impl DebugFragment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: FragmentId,
        fragment: Fragment,
        location: FragmentLocation,
        active_fragment: Option<FragmentId>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effects: &[Effect],
    ) -> Self {
        let state = if Some(id) == active_fragment {
            if is_in_innermost_active_function {
                DebugFragmentState::InnermostActiveFragment
            } else {
                DebugFragmentState::ActiveCaller
            }
        } else {
            DebugFragmentState::NotActive
        };

        let has_durable_breakpoint = source_map
            .fragment_to_instructions(&id)
            .iter()
            .any(|instruction| breakpoints.durable_at(instruction));

        let effect = effects.first().and_then(|effect| {
            if state.is_innermost_active_fragment() {
                Some(*effect)
            } else {
                None
            }
        });

        let data = DebugFragmentData {
            id,
            fragment: fragment.clone(),
            state,
            has_durable_breakpoint,
            effect,
        };
        let kind = DebugFragmentKind::new(
            fragment,
            location,
            active_fragment,
            is_in_innermost_active_function,
            cluster,
            fragments,
            source_map,
            breakpoints,
            effects,
        );

        Self { kind, data }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFragmentData {
    /// # The ID of the fragment that the `DebugFragment` was built from
    pub id: FragmentId,

    /// # The fragment that the `DebugFragment` was built from
    pub fragment: Fragment,

    /// # The static of the fragment
    pub state: DebugFragmentState,

    pub has_durable_breakpoint: bool,
    pub effect: Option<Effect>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugFragmentState {
    InnermostActiveFragment,
    ActiveCaller,
    NotActive,
}

impl DebugFragmentState {
    /// # Indicate whether this is the innermost active fragment
    ///
    /// The innermost active fragment is the active fragment in the innermost
    /// active function. The fragment where the process is currently stopped at.
    pub fn is_innermost_active_fragment(&self) -> bool {
        matches!(self, Self::InnermostActiveFragment)
    }

    /// # Indicate whether the fragment is active
    ///
    /// A fragment is active, either if the process is currently stopped here,
    /// or if it calls an active function (which is a function that contains an
    /// active fragment).
    pub fn is_active(&self) -> bool {
        matches!(self, Self::InnermostActiveFragment | Self::ActiveCaller)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugFragmentKind {
    CallToFunction { name: String },
    CallToFunctionRecursive { name: String },
    CallToHostFunction { name: String },
    CallToIntrinsic { name: String },
    Comment { text: String },
    Function { function: DebugFunction },
    ResolvedBinding { name: String },
    UnresolvedIdentifier { name: String },
    Value { as_string: String },
}

impl DebugFragmentKind {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fragment: Fragment,
        location: FragmentLocation,
        active_fragment: Option<FragmentId>,
        is_in_innermost_active_function: bool,
        cluster: &Cluster,
        fragments: &Fragments,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effects: &[Effect],
    ) -> Self {
        match fragment {
            Fragment::CallToFunction { name, .. } => {
                Self::CallToFunction { name }
            }
            Fragment::CallToFunctionRecursive { index, .. } => {
                let (called_function_id, _) = cluster
                    .functions
                    .get(&index)
                    .expect(
                    "The index of a recursive call must be valid within the \
                    calling function's cluster.",
                );
                let called_function = fragments
                    .get(called_function_id)
                    .expect(
                        "Expecting to find fragment referred to from a \
                        cluster.",
                    )
                    .as_function()
                    .expect(
                        "Got fragment ID through function call; must refer to \
                        a function.",
                    );
                let name = called_function.name.clone().expect(
                    "Calling anonymous functions recursively is not possible. \
                    Target of recursive function call must have a name.",
                );

                Self::CallToFunctionRecursive { name }
            }
            Fragment::CallToHostFunction { effect_number } => {
                let name = GameEngineHost::effect_number_to_function_name(
                    effect_number,
                )
                .expect("Expected effect number in code to be valid.")
                .to_string();

                Self::CallToHostFunction { name }
            }
            Fragment::CallToIntrinsic { intrinsic, .. } => {
                Self::CallToIntrinsic {
                    name: intrinsic.to_string(),
                }
            }
            Fragment::Comment { text } => Self::Comment {
                text: format!("# {text}"),
            },
            Fragment::Function { function } => {
                let function = DebugFunction::new(
                    function,
                    FunctionLocation::AnonymousFunction { location },
                    active_fragment,
                    is_in_innermost_active_function,
                    cluster,
                    fragments,
                    source_map,
                    breakpoints,
                    effects,
                );

                Self::Function { function }
            }
            Fragment::ResolvedBinding { name } => {
                Self::ResolvedBinding { name }
            }
            Fragment::UnresolvedIdentifier { name } => {
                Self::UnresolvedIdentifier { name }
            }
            Fragment::Value(value) => Self::Value {
                as_string: value.to_string(),
            },
        }
    }
}
