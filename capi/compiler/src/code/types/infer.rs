use crate::code::{syntax::Expression, FunctionCalls};

use super::{Signature, Type};

pub fn infer_expression(
    expression: &Expression,
    _: &FunctionCalls,
) -> Option<Signature> {
    match expression {
        Expression::LiteralNumber { .. } => Some(Signature {
            inputs: vec![],
            outputs: vec![Type::Number],
        }),
        _ => None,
    }
}
