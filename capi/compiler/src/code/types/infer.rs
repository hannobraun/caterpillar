use crate::code::{
    syntax::{Expression, MemberLocation},
    FunctionCalls,
};

use super::{Signature, Type};

pub fn infer_expression(
    expression: &Expression,
    _: &MemberLocation,
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
