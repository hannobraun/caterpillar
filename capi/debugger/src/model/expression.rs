use capi_compiler::{
    repr::{fragments::FragmentPayload, syntax},
    source_map::SourceMap2,
};
use capi_process::{EvaluatorEffect, Location, Process};

#[derive(Clone, Eq, PartialEq)]
pub struct Fragment {
    pub payload: FragmentPayload,
    pub location: Option<Location>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<EvaluatorEffect>,
}

impl Fragment {
    pub fn new(
        syntax_location: syntax::Location,
        payload: FragmentPayload,
        source_map: &SourceMap2,
        process: &Process,
    ) -> Self {
        let location = source_map.syntax_to_runtime(&syntax_location);

        let has_durable_breakpoint = if let Some(location) = &location {
            process.breakpoints().durable_at(location)
        } else {
            false
        };

        let is_comment = matches!(payload, FragmentPayload::Comment { .. });

        let effect =
            process.state().first_unhandled_effect().and_then(|effect| {
                let effect_location = source_map.runtime_to_syntax(
                    &process.state().most_recent_step().unwrap(),
                );

                if effect_location == syntax_location {
                    Some(effect.clone())
                } else {
                    None
                }
            });

        let is_on_call_stack = if let Some(location) = &location {
            process.stack().is_next_instruction_in_any_frame(location)
        } else {
            false
        };

        Self {
            payload,
            location,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        }
    }
}
