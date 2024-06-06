use crate::{
    runtime,
    syntax::{self, ExpressionKind},
    Program, ProgramEffect,
};

pub struct Expression {
    pub location: Option<runtime::InstructionAddress>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<ProgramEffect>,
}

impl Expression {
    pub fn new(expression: &syntax::Expression, program: &Program) -> Self {
        let location =
            program.source_map.location_to_address(&expression.location);

        let has_durable_breakpoint = if let Some(address) = location {
            program.breakpoints.durable_breakpoint_at(&address)
        } else {
            false
        };

        let is_comment =
            matches!(expression.kind, ExpressionKind::Comment { .. });

        let effect = program.effects.front().and_then(|effect| {
            let effect_location =
                program.source_map.address_to_location(&effect.address);

            if effect_location.as_ref() == Some(&expression.location) {
                Some(effect.clone())
            } else {
                None
            }
        });

        // This does not work reliably, for reasons I don't fully understand.
        // But it's better than nothing, so I'm keeping it for now, and will
        // hopefully fix it soon.
        let is_on_call_stack = location
            .map(|address| program.evaluator.call_stack().contains(address))
            .unwrap_or(false);

        Self {
            location,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        }
    }
}
