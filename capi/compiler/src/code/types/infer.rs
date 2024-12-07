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
            let host = function_calls.is_call_to_host_function(location);
            let intrinsic =
                function_calls.is_call_to_intrinsic_function(location);

            match (host, intrinsic) {
                (Some(host), None) => Some(host.signature.clone()),
                (None, Some(intrinsic)) => intrinsic.signature(),
                (None, None) => None,
                _ => {
                    unreachable!(
                        "Single identifier resolved to multiple function calls."
                    );
                }
            }
        }
        Expression::LiteralNumber { .. } => Some(Signature {
            inputs: vec![],
            outputs: vec![Type::Number],
        }),
        _ => None,
    }
}
