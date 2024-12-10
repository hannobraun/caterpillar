use std::{collections::BTreeMap, fmt};

use crate::{
    code::{
        syntax::{Branch, Expression, Located, MemberLocation, SyntaxTree},
        FunctionCalls,
    },
    intrinsics::IntrinsicFunction,
};

use super::{
    repr::{Signatures, Stack, Stacks},
    ExplicitTypes, Signature, Type,
};

pub fn infer_types(
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
) -> (Signatures, Stacks) {
    let mut signatures = BTreeMap::new();
    let mut stacks = BTreeMap::new();

    for function in syntax_tree.all_functions() {
        for branch in function.branches() {
            if let Err(TypeError {
                expected,
                actual,
                location,
            }) = infer_branch(
                branch,
                syntax_tree,
                explicit_types,
                function_calls,
                &mut signatures,
                &mut stacks,
            ) {
                let actual = actual
                    .map(|type_| format!("`{type_}`"))
                    .unwrap_or_else(|| "nothing".to_string());

                panic!(
                    "\n\
                    Type error: expected {expected}, got {actual}\n\
                    \n\
                    at {}\n",
                    location.display(syntax_tree),
                );
            }
        }
    }

    (signatures, stacks)
}

fn infer_branch(
    branch: Located<&Branch>,
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
    signatures: &mut Signatures,
    stacks: &mut Stacks,
) -> Result<(), TypeError> {
    let mut stack = Some(Vec::new());

    for expression in branch.expressions() {
        infer_expression(
            expression,
            syntax_tree,
            explicit_types,
            function_calls,
            signatures,
            stacks,
            &mut stack,
        )?;
    }

    Ok(())
}

fn infer_expression(
    expression: Located<&Expression>,
    syntax_tree: &SyntaxTree,
    explicit_types: &ExplicitTypes,
    function_calls: &FunctionCalls,
    signatures: &mut Signatures,
    stacks: &mut Stacks,
    stack: &mut Option<Stack>,
) -> Result<(), TypeError> {
    let explicit = explicit_types.signature_of(&expression.location);

    let inferred = match expression.fragment {
        Expression::Identifier { .. } => {
            let host =
                function_calls.is_call_to_host_function(&expression.location);
            let intrinsic = function_calls
                .is_call_to_intrinsic_function(&expression.location);

            match (host, intrinsic) {
                (Some(host), None) => Some(host.signature.clone()),
                (None, Some(intrinsic)) => {
                    infer_intrinsic(intrinsic, &expression.location, stack)?
                }
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
        if let Some(stack) = stack {
            for input in signature.inputs.iter().rev() {
                match stack.pop() {
                    Some(type_) if type_ == *input => {
                        // Type checks out!
                    }
                    actual => {
                        return Err(TypeError {
                            expected: ExpectedType::Specific(input.clone()),
                            actual,
                            location: expression.location,
                        });
                    }
                }
            }
            for output in signature.outputs.iter().cloned() {
                stack.push(output);
            }

            stacks.insert(expression.location.clone(), stack.clone());
        }

        signatures.insert(expression.location, signature);
    } else {
        *stack = None;
    }

    Ok(())
}

fn infer_intrinsic(
    intrinsic: &IntrinsicFunction,
    location: &MemberLocation,
    stack: &mut Option<Stack>,
) -> Result<Option<Signature>, TypeError> {
    let signature = match intrinsic {
        IntrinsicFunction::Eval => {
            let Some(stack) = stack.as_mut() else {
                return Ok(None);
            };

            match stack.last() {
                Some(Type::Function { signature }) => {
                    let outputs = signature.outputs.clone();
                    let inputs = signature
                        .inputs
                        .clone()
                        .into_iter()
                        .chain([Type::Function {
                            signature: signature.clone(),
                        }])
                        .collect();

                    Some(Signature { inputs, outputs })
                }
                actual => {
                    return Err(TypeError {
                        expected: ExpectedType::Function,
                        actual: actual.cloned(),
                        location: location.clone(),
                    });
                }
            }
        }
        intrinsic => intrinsic.signature(),
    };

    Ok(signature)
}

struct TypeError {
    expected: ExpectedType,
    actual: Option<Type>,
    location: MemberLocation,
}

enum ExpectedType {
    Function,
    Specific(Type),
}

impl fmt::Display for ExpectedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Specific(type_) => write!(f, "`{type_}`"),
        }
    }
}
