use capi_compiler::{
    repr::fragments::{Fragment, FragmentExpression, FragmentPayload},
    source_map::SourceMap,
};
use capi_process::{Effect, GameEngineHost, InstructionAddr, Process};

#[derive(Clone, Eq, PartialEq)]
pub struct Expression {
    pub expression: FragmentExpression,
    pub instruction: Option<InstructionAddr>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<Effect<GameEngineHost>>,
}

impl Expression {
    pub fn new(
        fragment: Fragment,
        source_map: &SourceMap,
        process: &Process,
    ) -> Option<Self> {
        let fragment_id = fragment.id();
        let FragmentPayload::Expression { expression, .. } = fragment.payload
        else {
            return None;
        };

        let instruction = source_map.fragment_to_instruction(&fragment_id);

        let has_durable_breakpoint = if let Some(instruction) = &instruction {
            process.breakpoints().durable_at(instruction)
        } else {
            false
        };

        let is_comment =
            matches!(expression, FragmentExpression::Comment { .. });

        let effect =
            process.state().first_unhandled_effect().and_then(|effect| {
                let effect_fragment = source_map.instruction_to_fragment(
                    &process.state().most_recent_step().unwrap(),
                );

                if effect_fragment == fragment_id {
                    Some(effect.clone())
                } else {
                    None
                }
            });

        let is_on_call_stack = if let Some(instruction) = &instruction {
            process
                .stack()
                .is_next_instruction_in_any_frame(instruction)
        } else {
            false
        };

        Some(Self {
            expression,
            instruction,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        })
    }
}
