use crate::{
    process::Process,
    runtime::{self, EvaluatorEffect},
    syntax::{self, ExpressionKind},
};

#[derive(Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: Option<runtime::Location>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<EvaluatorEffect>,
}

impl Expression {
    pub fn new(expression: syntax::Expression, process: &Process) -> Self {
        let location =
            process.source_map.syntax_to_runtime(&expression.location);

        let has_durable_breakpoint = if let Some(location) = &location {
            process.breakpoints.durable_breakpoint_at(location)
        } else {
            false
        };

        let is_comment =
            matches!(expression.kind, ExpressionKind::Comment { .. });

        let effect = process.effects.front().and_then(|effect| {
            let effect_location =
                process.source_map.runtime_to_syntax(&effect.location);

            if effect_location == expression.location {
                Some(effect.clone())
            } else {
                None
            }
        });

        let is_on_call_stack = if let Some(location) = &location {
            process.evaluator.stack().contains(location)
        } else {
            false
        };

        Self {
            kind: expression.kind.clone(),
            location,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        }
    }
}
