use capi_compiler::{
    code::{
        Fragment, FragmentLocation, FunctionCluster, FunctionLocation,
        NamedFunctions, Types,
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
        fragment: Fragment,
        location: FragmentLocation,
        active_fragment: Option<&FragmentLocation>,
        is_in_innermost_active_function: bool,
        cluster: &FunctionCluster,
        named_functions: &NamedFunctions,
        types: &Types,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        let signature = DebugFragmentSignature::new(&location, types);

        let state = if Some(&location) == active_fragment {
            if is_in_innermost_active_function {
                DebugFragmentState::InnermostActiveFragment
            } else {
                DebugFragmentState::ActiveCaller
            }
        } else {
            DebugFragmentState::NotActive
        };

        let has_durable_breakpoint = source_map
            .fragment_to_instructions(&location)
            .iter()
            .any(|instruction| breakpoints.durable_at(instruction));

        let active_effect = effect.and_then(|effect| {
            if state.is_innermost_active_fragment() {
                Some(*effect)
            } else {
                None
            }
        });

        let data = DebugFragmentData {
            fragment: fragment.clone(),
            location: location.clone(),
            signature,
            state,
            has_durable_breakpoint,
            effect: active_effect,
        };
        let kind = DebugFragmentKind::new(
            fragment,
            location,
            active_fragment,
            is_in_innermost_active_function,
            cluster,
            named_functions,
            types,
            source_map,
            breakpoints,
            effect,
        );

        Self { kind, data }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFragmentData {
    /// # The fragment that the `DebugFragment` was built from
    pub fragment: Fragment,

    /// # The location of the fragment
    pub location: FragmentLocation,

    /// # The type signature of the fragment
    pub signature: DebugFragmentSignature,

    /// # The state of the fragment
    pub state: DebugFragmentState,

    pub has_durable_breakpoint: bool,
    pub effect: Option<Effect>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFragmentSignature {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

impl DebugFragmentSignature {
    pub fn new(location: &FragmentLocation, types: &Types) -> Self {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        if let Some(signature) = types.for_fragments.get(location) {
            let convert = |index| {
                let type_ = types.inner.get(index).expect(
                    "Got type index from signature; must exist in `types`.",
                );
                format!("{type_:?}")
            };

            inputs.extend(signature.inputs.iter().map(convert));
            outputs.extend(signature.outputs.iter().map(convert));
        }

        Self { inputs, outputs }
    }
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
    Binding { name: String },
    CallToFunction { name: String },
    CallToFunctionRecursive { name: String },
    CallToHostFunction { name: String },
    CallToIntrinsic { name: String },
    Comment { text: String },
    Function { function: DebugFunction },
    UnresolvedIdentifier { name: String },
    Value { as_string: String },
}

impl DebugFragmentKind {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fragment: Fragment,
        location: FragmentLocation,
        active_fragment: Option<&FragmentLocation>,
        is_in_innermost_active_function: bool,
        cluster: &FunctionCluster,
        named_functions: &NamedFunctions,
        types: &Types,
        source_map: &SourceMap,
        breakpoints: &Breakpoints,
        effect: Option<&Effect>,
    ) -> Self {
        match fragment {
            Fragment::Binding { name, .. } => Self::Binding { name },
            Fragment::CallToUserDefinedFunction { hash, .. } => {
                let function = named_functions
                    .find_by_hash(&hash)
                    .expect("Expecting function referenced by call to exist.");
                let name = function.name.clone().expect(
                    "Got this function from search for named function. It must \
                    has a name.",
                );

                Self::CallToFunction { name }
            }
            Fragment::CallToUserDefinedFunctionRecursive { index, .. } => {
                let called_function_index = cluster
                    .functions
                    .get(&index)
                    .expect(
                    "The index of a recursive call must be valid within the \
                    calling function's cluster.",
                );
                let called_function =
                    named_functions.get(called_function_index).expect(
                        "Expecting to find fragment referred to from a \
                        cluster.",
                    );
                let name = called_function.name.clone().expect(
                    "Calling anonymous functions recursively is not possible. \
                    Target of recursive function call must have a name.",
                );

                Self::CallToFunctionRecursive { name }
            }
            Fragment::CallToHostFunction { number } => {
                let name = GameEngineHost
                    .function_by_number(number)
                    .expect("Expected effect number in code to be valid.")
                    .name()
                    .to_string();

                Self::CallToHostFunction { name }
            }
            Fragment::CallToIntrinsicFunction { intrinsic, .. } => {
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
                    named_functions,
                    types,
                    source_map,
                    breakpoints,
                    effect,
                );

                Self::Function { function }
            }
            Fragment::UnresolvedIdentifier { name, .. } => {
                Self::UnresolvedIdentifier { name }
            }
            Fragment::Value(value) => Self::Value {
                as_string: value.to_string(),
            },
        }
    }
}
