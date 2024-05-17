use crate::{ExpressionKind, InstructionAddress, Program, ProgramEffect};

pub struct Expression {
    pub address: Option<InstructionAddress>,
    pub has_durable_breakpoint: bool,
    pub is_comment: bool,
    pub effect: Option<ProgramEffect>,
}

impl Expression {
    pub fn new(expression: &crate::Expression, program: &Program) -> Self {
        let address =
            program.source_map.location_to_address(&expression.location);

        let has_durable_breakpoint = if let Some(address) = address {
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

        Self {
            address,
            has_durable_breakpoint,
            is_comment,
            effect,
        }
    }
}
