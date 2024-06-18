use crate::{
    breakpoints,
    process::Process,
    runtime::{self, EvaluatorEffect},
    source_map::SourceMap,
    syntax::{self, ExpressionKind},
};

#[derive(Clone, Eq, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: Option<runtime::Location>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<EvaluatorEffect>,
}

impl Expression {
    pub fn new(
        expression: syntax::Expression,
        source_map: &SourceMap,
        breakpoints: &breakpoints::State,
        process: &Process,
    ) -> Self {
        let location = source_map.syntax_to_runtime(&expression.location);

        let has_durable_breakpoint = if let Some(location) = &location {
            breakpoints.durable_at(location)
        } else {
            false
        };

        let is_comment =
            matches!(expression.kind, ExpressionKind::Comment { .. });

        let effect =
            process
                .state()
                .unhandled_effects
                .front()
                .and_then(|effect| {
                    let effect_location = source_map.runtime_to_syntax(
                        &process.state().most_recent_step().unwrap(),
                    );

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
            kind: expression.kind,
            location,
            has_durable_breakpoint,
            is_comment,
            is_on_call_stack,
            effect,
        }
    }
}
