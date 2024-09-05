use capi_compiler::{
    fragments::{Fragment, FragmentId, FragmentKind, Fragments, Payload},
    host::Host,
    source_map::SourceMap,
};
use capi_game_engine::host::GameEngineHost;
use capi_process::{Effect, InstructionAddress, Process};

use super::DebugFunction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFragment {
    pub kind: DebugFragmentKind,

    /// # Indicate whether the expression is active
    ///
    /// An expression is active, either if it is currently being executed, or if
    /// it calls an active function.
    pub is_active: bool,

    pub has_durable_breakpoint: bool,
    pub first_instruction: Option<InstructionAddress>,
    pub effect: Option<Effect>,
}

impl DebugFragment {
    pub fn new(
        fragment: Fragment,
        active_fragment: Option<FragmentId>,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Option<Self> {
        let instructions = source_map.fragment_to_instructions(&fragment.id());

        let is_active = Some(fragment.id()) == active_fragment;

        let has_durable_breakpoint = if let Some(instructions) = &instructions {
            instructions.iter().any(|instruction| {
                process.breakpoints().durable_at(instruction)
            })
        } else {
            false
        };

        let effect = process.inspect_effect().and_then(|effect| {
            let effect_fragment = source_map
                .instruction_to_fragment(&process.most_recent_step().unwrap())
                .expect("Expecting effects to originate from user code.");

            if effect_fragment == fragment.id() {
                Some(*effect)
            } else {
                None
            }
        });

        let kind = DebugFragmentKind::new(
            fragment,
            active_fragment,
            fragments,
            source_map,
            process,
        )?;

        Some(Self {
            kind,
            is_active,
            has_durable_breakpoint,
            first_instruction: instructions
                .and_then(|instruction| instruction.first().copied()),
            effect,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugFragmentKind {
    CallToFunction {
        name: String,
    },
    CallToHostFunction {
        name: String,
    },
    CallToIntrinsic {
        name: String,
    },
    Comment {
        text: String,
    },
    Function {
        function: DebugFunction,
    },
    ResolvedBinding {
        name: String,
    },

    /// # An expression, not covered by other variants
    ///
    /// ## Implementation Note
    ///
    /// This shouldn't exist. We should split out all the relevant variants, so
    /// the UI code can match on them. This would also pave the way for syntax
    /// highlighting in the debugger.
    OtherExpression(Payload),
}

impl DebugFragmentKind {
    pub fn new(
        fragment: Fragment,
        active_fragment: Option<FragmentId>,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Option<Self> {
        let FragmentKind::Payload { payload, .. } = fragment.kind else {
            return None;
        };

        let kind = match payload {
            Payload::CallToFunction { name, .. } => {
                Self::CallToFunction { name }
            }
            Payload::CallToHostFunction { effect_number } => {
                let name = GameEngineHost::effect_number_to_function_name(
                    effect_number,
                )
                .expect("Expected effect number in code to be valid.")
                .to_string();

                Self::CallToHostFunction { name }
            }
            Payload::CallToIntrinsic { intrinsic, .. } => {
                Self::CallToIntrinsic {
                    name: intrinsic.to_string(),
                }
            }
            Payload::Comment { text } => Self::Comment {
                text: format!("# {text}"),
            },
            Payload::Function { function } => {
                let function = DebugFunction::new(
                    function,
                    active_fragment,
                    fragments,
                    source_map,
                    process,
                );

                Self::Function { function }
            }
            Payload::ResolvedBinding { name } => Self::ResolvedBinding { name },
            payload => Self::OtherExpression(payload),
        };

        Some(kind)
    }
}
