use std::collections::BTreeMap;

use crate::code::{
    syntax::{Branch, Expression, Located, SyntaxTree},
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
            infer_branch(
                branch,
                syntax_tree,
                explicit_types,
                function_calls,
                &mut types,
            );
        }
    }

    types
}

pub fn infer_branch(
    branch: Located<&Branch>,
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
    types: &mut TypesInner,
) {
    let mut stack = Some(Vec::new());

    for expression in branch.expressions() {
        infer_expression(
            expression,
            syntax_tree,
            explicit_types,
            function_calls,
            types,
            &mut stack,
        );
    }
}

pub fn infer_expression(
    expression: Located<&Expression>,
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
    types: &mut TypesInner,
    stack: &mut Option<Vec<Type>>,
) {
    let explicit = explicit_types.signature_of(&expression.location);

    let inferred = match expression.fragment {
        Expression::Identifier { .. } => {
            let host =
                function_calls.is_call_to_host_function(&expression.location);
            let intrinsic = function_calls
                .is_call_to_intrinsic_function(&expression.location);

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
            "Type that could be inferred was also specified explicitly. This \
            is currently not allowed, as the goal is to transition away from \
            explicit type annotations completely.\n\
            \n\
            Explicit type: {explicit:?}\n\
            Inferred type: {inferred:?}\n\
            \n\
            At {}\n",
            expression.location.display(syntax_tree),
        );
    }

    if let Some(signature) = inferred.or(explicit.cloned()) {
        types.insert(expression.location, signature);
    } else {
        *stack = None;
    }
}
