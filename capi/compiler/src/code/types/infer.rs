use crate::code::syntax::Expression;

use super::{Signature, Type};

pub fn infer_expression(expression: &Expression) -> Option<Signature> {
    match expression {
        Expression::LiteralNumber { .. } => Some(Signature {
            inputs: vec![],
            outputs: vec![Type::Number],
        }),
        _ => None,
    }
}
