use crate::{InstructionAddress, Program};

pub struct Expression {
    pub address: Option<InstructionAddress>,
}

impl Expression {
    pub fn new(expression: &crate::Expression, program: &Program) -> Self {
        Self {
            address: program
                .source_map
                .location_to_address(&expression.location),
        }
    }
}
