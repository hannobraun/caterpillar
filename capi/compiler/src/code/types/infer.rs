use std::collections::BTreeMap;

use crate::code::{
    syntax::{Expression, MemberLocation, SyntaxTree},
    FunctionCalls,
};

use super::{repr::TypesInner, ExplicitTypes, Signature, Type};

pub fn infer_types(
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
) -> TypesInner {
    let mut types = BTreeMap::new();

    for function in syntax_tree.all_functions() {
        for branch in function.branches() {
            for expression in branch.expressions() {
                let inferred = infer_expression(
                    expression.fragment,
                    &expression.location,
                    syntax_tree,
                    explicit_types,
                    function_calls,
                    &mut types,
                );

                if let Some(signature) = inferred {
                    types.insert(expression.location, signature);
                }
            }
        }
    }

    types
}

pub fn infer_expression(
    expression: &Expression,
    location: &MemberLocation,
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
    _: &mut TypesInner,
) -> Option<Signature> {
    let explicit = explicit_types.signature_of(location);

    let inferred = match expression {
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
    };

    if let (Some(explicit), Some(inferred)) = (explicit, inferred.as_ref()) {
        panic!(
            "Type that could be inferred was also specified \
                        explicitly. This is currently not allowed, as the goal \
                        is to transition away from explicit type annotations \
                        completely.\n\
                        \n\
                        Explicit type: {explicit:?}\n\
                        Inferred type: {inferred:?}\n\
                        \n\
                        At {}\n",
            location.display(syntax_tree),
        );
    }

    inferred.or(explicit.cloned())
}
