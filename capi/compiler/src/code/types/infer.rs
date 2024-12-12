use std::fmt;

use crate::{
    code::{
        syntax::{Branch, Expression, Located, MemberLocation, SyntaxTree},
        FunctionCalls, Index, IndexMap,
    },
    intrinsics::IntrinsicFunction,
};

use super::{
    repr::{Signatures, Stack, Stacks},
    ExplicitTypes, Signature, Type,
};

pub fn infer_types(context: Context) -> InferenceOutput {
    let mut output = InferenceOutput::default();

    for function in context.syntax_tree.all_functions() {
        for branch in function.branches() {
            if let Err(TypeError {
                expected,
                actual,
                location,
            }) = infer_branch(branch, context, &mut output)
            {
                let actual = actual
                    .map(|type_| format!("`{type_}`"))
                    .unwrap_or_else(|| "nothing".to_string());

                panic!(
                    "\n\
                    Type error: expected {expected}, got {actual}\n\
                    \n\
                    at {}\n",
                    location.display(context.syntax_tree),
                );
            }
        }
    }

    output
}

#[derive(Clone, Copy)]
pub struct Context<'r> {
    pub syntax_tree: &'r SyntaxTree,
    pub function_calls: &'r FunctionCalls,
    pub explicit_types: &'r ExplicitTypes,
}

#[derive(Default)]
pub struct InferenceOutput {
    pub signatures: Signatures,
    pub stacks: Stacks,
}

fn infer_branch(
    branch: Located<&Branch>,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<(), TypeError> {
    let mut local_types = LocalTypes::default();
    let mut local_stack = LocalStack::default();

    for expression in branch.expressions() {
        infer_expression(
            expression,
            &mut local_types,
            &mut local_stack,
            context,
            output,
        )?;
    }

    Ok(())
}

fn infer_expression(
    expression: Located<&Expression>,
    local_types: &mut LocalTypes,
    local_stack: &mut LocalStack,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<(), TypeError> {
    let explicit = context
        .explicit_types
        .signature_of(&expression.location)
        .cloned()
        .map(|signature| make_indirect(signature, local_types));

    let inferred = match expression.fragment {
        Expression::Identifier { .. } => {
            let host = context
                .function_calls
                .is_call_to_host_function(&expression.location);
            let intrinsic = context
                .function_calls
                .is_call_to_intrinsic_function(&expression.location);

            match (host, intrinsic) {
                (Some(host), None) => {
                    let signature =
                        make_indirect(host.signature.clone(), local_types);
                    Some(signature)
                }
                (None, Some(intrinsic)) => {
                    let signature = infer_intrinsic(
                        intrinsic,
                        &expression.location,
                        local_stack,
                    )?;

                    signature
                        .map(|signature| make_indirect(signature, local_types))
                }
                (None, None) => None,
                _ => {
                    unreachable!("Single identifier resolved multiple times.");
                }
            }
        }
        Expression::LiteralNumber { .. } => {
            let signature = Signature {
                inputs: vec![],
                outputs: vec![Type::Number],
            };
            let signature = make_indirect(signature, local_types);
            Some(signature)
        }
        _ => None,
    };

    if let (Some(explicit), Some(inferred)) =
        (explicit.as_ref(), inferred.as_ref())
    {
        panic!(
            "Type that could be inferred was also specified explicitly. This \
            is currently not allowed, as the goal is to transition away from \
            explicit type annotations completely.\n\
            \n\
            Explicit type: {explicit:?}\n\
            Inferred type: {inferred:?}\n\
            \n\
            At {}\n",
            expression.location.display(context.syntax_tree),
        );
    }

    if let Some(signature) = inferred.or(explicit) {
        if let Some(local_stack) = local_stack.get_mut() {
            for input_index in signature.inputs.iter().rev() {
                let input = local_types.get(input_index);

                match local_stack.pop() {
                    Some(operand) => {
                        match input {
                            InferredType::Known(input) => {
                                if operand == *input {
                                    // Type checks out!
                                } else {
                                    return Err(TypeError {
                                        expected: ExpectedType::Specific(
                                            input.clone(),
                                        ),
                                        actual: Some(operand),
                                        location: expression.location,
                                    });
                                }
                            }
                        }
                    }
                    None => {
                        let expected = match input {
                            InferredType::Known(input) => {
                                ExpectedType::Specific(input.clone())
                            }
                        };

                        return Err(TypeError {
                            expected,
                            actual: None,
                            location: expression.location,
                        });
                    }
                }
            }
            for output in signature.outputs.iter() {
                let InferredType::Known(output) = local_types.get(output);
                local_stack.push(output.clone());
            }

            output
                .stacks
                .insert(expression.location.clone(), local_stack.clone());
        }

        if let Some(signature) = make_direct(signature, local_types) {
            output.signatures.insert(expression.location, signature);
        }
    } else {
        local_stack.invalidate();
    }

    Ok(())
}

fn infer_intrinsic(
    intrinsic: &IntrinsicFunction,
    location: &MemberLocation,
    local_stack: &mut LocalStack,
) -> Result<Option<Signature>, TypeError> {
    let signature = match intrinsic {
        IntrinsicFunction::Eval => {
            let Some(local_stack) = local_stack.get_mut() else {
                return Ok(None);
            };

            match local_stack.last() {
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

fn make_indirect(
    signature: Signature,
    local_types: &mut LocalTypes,
) -> Signature<Index<InferredType>> {
    let mut map = |from: Vec<Type>| {
        from.into_iter()
            .map(|type_| local_types.push(InferredType::Known(type_)))
            .collect()
    };

    Signature {
        inputs: map(signature.inputs),
        outputs: map(signature.outputs),
    }
}

fn make_direct(
    signature: Signature<Index<InferredType>>,
    local_types: &LocalTypes,
) -> Option<Signature<Type>> {
    let try_map = |from: Vec<Index<InferredType>>| {
        from.into_iter()
            .map(|index| local_types.get(&index).clone().into_type())
            .collect::<Option<_>>()
    };

    let inputs = try_map(signature.inputs)?;
    let outputs = try_map(signature.outputs)?;

    Some(Signature { inputs, outputs })
}

struct TypeError {
    expected: ExpectedType,
    actual: Option<Type>,
    location: MemberLocation,
}

#[derive(Default)]
struct LocalTypes {
    inner: IndexMap<InferredType>,
}

impl LocalTypes {
    fn push(&mut self, type_: InferredType) -> Index<InferredType> {
        self.inner.push(type_)
    }

    fn get(&self, index: &Index<InferredType>) -> &InferredType {
        let Some(type_) = self.inner.get(index) else {
            unreachable!(
                "We're never removing any local types. Any index must be valid."
            );
        };

        type_
    }
}

struct LocalStack {
    inner: Option<Stack>,
}
impl LocalStack {
    fn get_mut(&mut self) -> Option<&mut Stack> {
        self.inner.as_mut()
    }

    fn invalidate(&mut self) {
        self.inner = None;
    }
}

impl Default for LocalStack {
    fn default() -> Self {
        Self {
            inner: Some(Vec::new()),
        }
    }
}

#[derive(Clone, Debug)]
enum InferredType {
    Known(Type),
}

impl InferredType {
    pub fn into_type(self) -> Option<Type> {
        match self {
            Self::Known(type_) => Some(type_),
        }
    }
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
