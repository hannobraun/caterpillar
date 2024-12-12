use std::{collections::BTreeMap, fmt};

use crate::{
    code::{
        syntax::{Branch, Expression, Located, MemberLocation, SyntaxTree},
        Bindings, FunctionCalls, Index, IndexMap,
    },
    intrinsics::IntrinsicFunction,
};

use super::{
    repr::{Signatures, Stacks},
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
    pub bindings: &'r Bindings,
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

    let mut signatures = BTreeMap::new();

    for expression in branch.expressions() {
        let location = expression.location.clone();

        let signature = infer_expression(
            expression,
            &mut local_types,
            &mut local_stack,
            context,
            output,
        )?;

        if let Some(signature) = signature {
            signatures.insert(location, signature);
        }
    }

    Ok(())
}

fn infer_expression(
    expression: Located<&Expression>,
    local_types: &mut LocalTypes,
    local_stack: &mut LocalStack,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<Option<Signature<Index<InferredType>>>, TypeError> {
    let explicit = context
        .explicit_types
        .signature_of(&expression.location)
        .cloned()
        .map(|signature| make_indirect(signature, local_types));

    let inferred = match expression.fragment {
        Expression::Identifier { .. } => {
            let binding = context.bindings.is_binding(&expression.location);
            let host = context
                .function_calls
                .is_call_to_host_function(&expression.location);
            let intrinsic = context
                .function_calls
                .is_call_to_intrinsic_function(&expression.location);

            match (binding, host, intrinsic) {
                (Some(_binding), None, None) => {
                    let output = local_types.push(InferredType::Unknown);
                    let signature = Signature {
                        inputs: vec![],
                        outputs: vec![output],
                    };
                    Some(signature)
                }
                (None, Some(host), None) => {
                    let signature =
                        make_indirect(host.signature.clone(), local_types);
                    Some(signature)
                }
                (None, None, Some(intrinsic)) => {
                    let signature = infer_intrinsic(
                        intrinsic,
                        &expression.location,
                        local_types,
                        local_stack,
                    )?;

                    signature
                        .map(|signature| make_indirect(signature, local_types))
                }
                (None, None, None) => None,
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

    if let [Some(explicit), Some(inferred)] =
        [explicit.as_ref(), inferred.as_ref()].map(|signature| {
            signature.and_then(|signature| make_direct(signature, local_types))
        })
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
                let input = local_types.get(input_index).clone();

                match local_stack.pop() {
                    Some(operand_index) => {
                        let operand = local_types.get(&operand_index).clone();

                        match (operand, input) {
                            (
                                InferredType::Known(operand),
                                InferredType::Known(input),
                            ) => {
                                if operand == input {
                                    // Type checks out!
                                } else {
                                    return Err(TypeError {
                                        expected: ExpectedType::Specific(input),
                                        actual: Some(operand),
                                        location: expression.location,
                                    });
                                }
                            }
                            (
                                InferredType::Known(_operand),
                                InferredType::Unknown,
                            ) => {
                                // We could infer the type of the binding here.
                            }
                            (
                                InferredType::Unknown,
                                InferredType::Known(_input),
                            ) => {
                                // We could infer the type of the operand here.
                            }
                            (InferredType::Unknown, InferredType::Unknown) => {
                                // We could unify the two types here, to make
                                // sure that if one gets inferred, the other is
                                // known too.
                            }
                        }
                    }
                    None => {
                        let expected = match input {
                            InferredType::Known(input) => {
                                ExpectedType::Specific(input)
                            }
                            InferredType::Unknown => ExpectedType::Unknown,
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
                local_stack.push(*output);
            }

            local_stack
                .iter()
                .map(|index| local_types.get(index).clone().into_type())
                .collect::<Option<Vec<_>>>()
                .into_iter()
                .for_each(|local_stack| {
                    output.stacks.insert(
                        expression.location.clone(),
                        local_stack.clone(),
                    );
                });
        }

        if let Some(signature) = make_direct(&signature, local_types) {
            output.signatures.insert(expression.location, signature);
        }

        return Ok(Some(signature));
    } else {
        local_stack.invalidate();
    }

    Ok(None)
}

fn infer_intrinsic(
    intrinsic: &IntrinsicFunction,
    location: &MemberLocation,
    local_types: &mut LocalTypes,
    local_stack: &mut LocalStack,
) -> Result<Option<Signature>, TypeError> {
    let signature = match intrinsic {
        IntrinsicFunction::Eval => {
            let Some(local_stack) = local_stack.get_mut() else {
                return Ok(None);
            };

            let top_operand =
                local_stack.last().map(|index| local_types.get(index));

            match top_operand {
                Some(InferredType::Known(Type::Function { signature })) => {
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
                Some(InferredType::Known(actual)) => {
                    return Err(TypeError {
                        expected: ExpectedType::Function,
                        actual: Some(actual.clone()),
                        location: location.clone(),
                    });
                }
                Some(InferredType::Unknown) => None,
                None => {
                    return Err(TypeError {
                        expected: ExpectedType::Function,
                        actual: None,
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
    signature: &Signature<Index<InferredType>>,
    local_types: &LocalTypes,
) -> Option<Signature<Type>> {
    let try_map = |from: &Vec<Index<InferredType>>| {
        from.iter()
            .map(|index| local_types.get(index).clone().into_type())
            .collect::<Option<_>>()
    };

    let inputs = try_map(&signature.inputs)?;
    let outputs = try_map(&signature.outputs)?;

    Some(Signature { inputs, outputs })
}

struct TypeError {
    expected: ExpectedType,
    actual: Option<Type>,
    location: MemberLocation,
}

#[derive(Debug, Default)]
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

#[derive(Debug)]
struct LocalStack {
    inner: Option<Vec<Index<InferredType>>>,
}
impl LocalStack {
    fn get_mut(&mut self) -> Option<&mut Vec<Index<InferredType>>> {
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
    Unknown,
}

impl InferredType {
    pub fn into_type(self) -> Option<Type> {
        match self {
            Self::Known(type_) => Some(type_),
            Self::Unknown => None,
        }
    }
}

enum ExpectedType {
    Function,
    Specific(Type),
    Unknown,
}

impl fmt::Display for ExpectedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Specific(type_) => write!(f, "`{type_}`"),
            Self::Unknown => write!(f, "unknown type"),
        }
    }
}
