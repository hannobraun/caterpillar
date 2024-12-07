use crate::code::{
    syntax::{Expression, MemberLocation},
    FunctionCalls,
};

use super::{Signature, Type};

pub fn infer_expression(
    expression: &Expression,
    location: &MemberLocation,
    function_calls: &FunctionCalls,
) -> Option<Signature> {
    match expression {
        Expression::Identifier { .. } => {
            let intrinsic =
                function_calls.is_call_to_intrinsic_function(location);

            match intrinsic {
                Some(intrinsic) => intrinsic.signature(),
                None => None,
            }
        }
        Expression::LiteralNumber { .. } => Some(Signature {
            inputs: vec![],
            outputs: vec![Type::Number],
        }),
        _ => None,
    }
}
