use crate::{
    runtime,
    syntax::{self, ExpressionKind},
    Program, ProgramEffect,
};

pub struct Expression {
    pub location: runtime::InstructionAddress,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<ProgramEffect>,
}

impl Expression {
    pub fn new(expression: &syntax::Expression, program: &Program) -> Self {
        let location =
            program.source_map.syntax_to_runtime(&expression.location);

        let has_durable_breakpoint =
            program.breakpoints.durable_breakpoint_at(&location);

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

        let is_on_call_stack =
            program.evaluator.call_stack().contains(location);

        Self {
            location,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        }
    }
}
