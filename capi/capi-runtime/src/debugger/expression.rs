use crate::{InstructionAddress, Program};

pub struct Expression {
    pub address: Option<InstructionAddress>,
    pub has_durable_breakpoint: bool,
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

        Self {
            address,
            has_durable_breakpoint,
        }
    }
}
