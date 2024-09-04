use capi_compiler::{
    fragments::{self, Fragment, FragmentId, FragmentKind, Fragments},
    source_map::SourceMap,
};
use capi_process::{Effect, InstructionAddress, Process};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFragment {
    pub kind: ExpressionKind,

    /// # Indicate whether the expression is active
    ///
    /// An expression is active, either if it is currently being executed, or if
    /// it calls an active function.
    pub is_active: bool,

    pub has_durable_breakpoint: bool,
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

        let kind = ExpressionKind::new(
            fragment,
            active_fragment,
            instructions,
            fragments,
            source_map,
            process,
        )?;

        Some(Self {
            kind,
            is_active,
            has_durable_breakpoint,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionKind {
    Comment { text: String },
    Function { function: Function },
    Other(OtherExpression),
}

impl ExpressionKind {
    pub fn new(
        fragment: Fragment,
        active_fragment: Option<FragmentId>,
        instructions: Option<&Vec<InstructionAddress>>,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Option<Self> {
        let fragment_id = fragment.id();
        let FragmentKind::Payload { payload, .. } = fragment.kind else {
            return None;
        };

        if let fragments::Payload::Function { function } = payload {
            let function = Function::new(
                function,
                active_fragment,
                fragments,
                source_map,
                process,
            );

            return Some(Self::Function { function });
        }
        if let fragments::Payload::Comment { text } = payload {
            return Some(Self::Comment {
                text: format!("# {text}"),
            });
        }

        let effect = process.inspect_effect().and_then(|effect| {
            let effect_fragment = source_map
                .instruction_to_fragment(&process.most_recent_step().unwrap())
                .expect("Expecting effects to originate from user code.");

            if effect_fragment == fragment_id {
                Some(*effect)
            } else {
                None
            }
        });

        Some(Self::Other(OtherExpression {
            payload,
            first_instruction: instructions
                .and_then(|instruction| instruction.first().copied()),
            effect,
        }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OtherExpression {
    pub payload: fragments::Payload,
    pub first_instruction: Option<InstructionAddress>,
    pub effect: Option<Effect>,
}
