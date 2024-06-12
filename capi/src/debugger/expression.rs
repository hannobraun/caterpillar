use crate::{
    program::{Program, ProgramEffect},
    runtime,
    syntax::{self, ExpressionKind},
};

pub struct Expression {
    pub location: Option<runtime::Location>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<ProgramEffect>,
}

impl Expression {
    pub fn new(expression: &syntax::Expression, program: &Program) -> Self {
        let location =
            program.source_map.syntax_to_runtime(&expression.location);

        let has_durable_breakpoint = if let Some(location) = &location {
            program.breakpoints.durable_breakpoint_at(location)
        } else {
            false
        };

        let is_comment =
            matches!(expression.kind, ExpressionKind::Comment { .. });

        let effect = program.effects.front().and_then(|effect| {
            let effect_location =
                program.source_map.runtime_to_syntax(&effect.location);

            if effect_location == expression.location {
                Some(effect.clone())
            } else {
                None
            }
        });

        let is_on_call_stack = if let Some(location) = &location {
            program.evaluator.stack().contains(location)
        } else {
            false
        };

        Self {
            location,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        }
    }
}
