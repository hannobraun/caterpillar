use std::{
    collections::BTreeMap,
    fmt::{self, Write},
    result,
};

use crate::{
    code::{
        bindings::Environment,
        syntax::{
            Branch, Expression, FunctionLocation, Located, MemberLocation,
            Pattern, SyntaxTree,
        },
        Binding, Bindings, Dependencies, FunctionCalls, Index, IndexMap,
    },
    intrinsics::IntrinsicFunction,
};

use super::{repr::Stacks, Signature, Type, TypeAnnotations};

pub fn infer_types(context: Context) -> InferenceOutput {
    let mut output = InferenceOutput::default();

    for function in context.dependencies.functions(context.syntax_tree) {
        let environment = context.bindings.environment_of(&function.location);

        for branch in function.branches() {
            match infer_branch(branch, environment, context, &mut output) {
                Ok(signature) => {
                    if let Some(signature) = signature {
                        output
                            .functions
                            .insert(function.location.clone(), signature);
                    }
                }
                Err(TypeError {
                    expected,
                    actual,
                    location,
                }) => {
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
    }

    output
}

#[derive(Clone, Copy)]
pub struct Context<'r> {
    pub syntax_tree: &'r SyntaxTree,
    pub bindings: &'r Bindings,
    pub function_calls: &'r FunctionCalls,
    pub dependencies: &'r Dependencies,
    pub annotations: &'r TypeAnnotations,
}

#[derive(Default)]
pub struct InferenceOutput {
    pub functions: BTreeMap<FunctionLocation, Signature>,
    pub expressions: BTreeMap<MemberLocation, Signature>,
    pub bindings: BTreeMap<Binding, Type>,
    pub stacks: Stacks,
}

fn infer_branch(
    branch: Located<&Branch>,
    environment: &Environment,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<Option<Signature>> {
    let mut local_types = LocalTypes::default();
    let mut local_stack = LocalStack::default();

    let bindings = branch
        .bindings()
        .map(|(_, binding)| binding)
        .chain(environment.values().cloned())
        .map(|binding| {
            let type_ = output
                .bindings
                .get(&binding)
                .cloned()
                .map(InferredType::Known)
                .unwrap_or(InferredType::Unknown);
            let type_ = local_types.push(type_);
            (binding, type_)
        })
        .collect();

    let mut signatures = BTreeMap::new();
    let mut stacks = BTreeMap::new();

    for expression in branch.expressions() {
        let location = expression.location.clone();

        if let Some(stack) = local_stack.get_mut().cloned() {
            stacks.insert(location.clone(), stack);
        }

        let signature = infer_expression(
            expression,
            &bindings,
            &output.functions,
            &mut local_types,
            &mut local_stack,
            context,
        )?;

        if let Some(signature) = signature {
            signatures.insert(location, signature);
        }
    }

    // Information from a later expression could have allowed us to infer the
    // type of an earlier one. So let's handle the signatures we collected
    // _after_ we look at all of the expressions.
    for (location, signature) in signatures {
        if let Some(signature) = make_signature_direct(&signature, &local_types)
        {
            output.expressions.insert(location, signature);
        }
    }
    for (location, local_stack) in stacks {
        let Some(local_stack) = make_stack_direct(&local_stack, &local_types)
        else {
            continue;
        };

        output.stacks.insert(location, local_stack);
    }

    let signature =
        infer_branch_signature(branch, bindings, local_types, local_stack);

    Ok(signature)
}

fn infer_expression(
    expression: Located<&Expression>,
    bindings: &BTreeMap<Binding, Index<InferredType>>,
    functions: &BTreeMap<FunctionLocation, Signature>,
    local_types: &mut LocalTypes,
    local_stack: &mut LocalStack,
    context: Context,
) -> Result<Option<Signature<Index<InferredType>>>> {
    let explicit = context
        .annotations
        .signature_of(&expression.location)
        .cloned()
        .map(|signature| make_signature_indirect(signature, local_types));

    let inferred = match expression.fragment {
        Expression::Identifier { name } => {
            let binding = context.bindings.is_binding(&expression.location);
            let host = context
                .function_calls
                .is_call_to_host_function(&expression.location);
            let intrinsic = context
                .function_calls
                .is_call_to_intrinsic_function(&expression.location);
            let user_defined = context
                .function_calls
                .is_call_to_user_defined_function(&expression.location);

            match (binding, host, intrinsic, user_defined) {
                (Some(binding), None, None, None) => {
                    let Some(output) = bindings.get(binding).copied() else {
                        let Binding {
                            identifier_index,
                            branch,
                        } = binding;

                        let mut available_bindings = String::new();
                        for (binding, type_) in bindings {
                            let Binding {
                                identifier_index,
                                branch,
                            } = binding;
                            let type_ = local_types.get(type_);
                            write!(
                                available_bindings,
                                "- index `{identifier_index}` at {}: {type_:?}",
                                branch.display(context.syntax_tree),
                            )
                            .expect("Writing to `String` can not fail.");
                        }

                        unreachable!(
                            "Identifier `{name}` has been resolved as binding, \
                            but it is not known in the branch.\n\
                            \n\
                            at {}\n\
                            \n\
                            Binding: identifier index `{identifier_index}` at \
                            {}\n\
                            \n\
                            Available bindings in branch:\n\
                            {available_bindings}",
                            expression.location.display(context.syntax_tree),
                            branch.display(context.syntax_tree),
                        );
                    };
                    let signature = Signature {
                        inputs: vec![],
                        outputs: vec![output],
                    };
                    Some(signature)
                }
                (None, Some(host), None, None) => {
                    let signature = make_signature_indirect(
                        host.signature.clone(),
                        local_types,
                    );
                    Some(signature)
                }
                (None, None, Some(intrinsic), None) => {
                    let signature = infer_intrinsic(
                        intrinsic,
                        &expression.location,
                        local_types,
                        local_stack,
                    )?;

                    signature.map(|signature| {
                        make_signature_indirect(signature, local_types)
                    })
                }
                (None, None, None, Some(user_defined)) => {
                    functions.get(user_defined).map(|signature| {
                        make_signature_indirect(signature.clone(), local_types)
                    })
                }
                (None, None, None, None) => None,
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
            let signature = make_signature_indirect(signature, local_types);
            Some(signature)
        }
        Expression::LocalFunction { .. } => {
            let location = FunctionLocation::from(expression.location.clone());
            functions.get(&location).map(|signature| {
                let signature = Signature {
                    inputs: vec![],
                    outputs: vec![Type::Function {
                        signature: signature.clone(),
                    }],
                };
                make_signature_indirect(signature, local_types)
            })
        }
    };

    // This is not redundant with the check below, where the two signatures are
    // merged. That check only looks for conflicting types, while this one
    // disallows type annotations that can be fully inferred.
    if let [Some(explicit), Some(inferred)] =
        [explicit.as_ref(), inferred.as_ref()].map(|signature| {
            signature.and_then(|signature| {
                make_signature_direct(signature, local_types)
            })
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
            Expression: {:#?}
            \n\
            At {}\n",
            expression.fragment,
            expression.location.display(context.syntax_tree),
        );
    }

    let signature = match (inferred, explicit) {
        (Some(inferred), Some(explicit)) => {
            let merge = |a: &Vec<Index<InferredType>>, b| {
                let mut indices = Vec::new();

                for (a, b) in a.iter().zip(b) {
                    let index = match [a, b].map(|index| local_types.get(index))
                    {
                        [InferredType::Known(_), InferredType::Known(_)] => {
                            if a == b {
                                a
                            } else {
                                panic!(
                                    "Explicit type annotation conflicts with \
                                    inferred type.\n\
                                    \n\
                                    Explicit type: {explicit:?}\n\
                                    Inferred type: {inferred:?}\n\
                                    \n\
                                    At {}\n",
                                    expression
                                        .location
                                        .display(context.syntax_tree),
                                );
                            }
                        }
                        [InferredType::Known(_), InferredType::Unknown] => a,
                        [InferredType::Unknown, InferredType::Known(_)] => b,
                        [InferredType::Unknown, InferredType::Unknown] => a,
                    };

                    indices.push(*index);
                }

                indices
            };

            Some(Signature {
                inputs: merge(&inferred.inputs, &explicit.inputs),
                outputs: merge(&inferred.outputs, &explicit.outputs),
            })
        }
        (inferred, explicit) => inferred.or(explicit),
    };

    if let Some(signature) = signature {
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
                                InferredType::Known(input),
                            ) => {
                                local_types.inner.insert(
                                    operand_index,
                                    InferredType::Known(input),
                                );
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
) -> Result<Option<Signature>> {
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

fn infer_branch_signature(
    branch: Located<&Branch>,
    bindings: BTreeMap<Binding, Index<InferredType>>,
    local_types: LocalTypes,
    local_stack: LocalStack,
) -> Option<Signature> {
    let inputs = branch
        .parameters
        .iter()
        .map(|parameter| match parameter {
            Pattern::Identifier { name } => {
                let Some(binding) = branch
                    .bindings()
                    .find_map(|(n, binding)| (n == *name).then_some(binding))
                else {
                    unreachable!(
                        "Parameter of branch not recognized as a binding."
                    );
                };

                let Some(type_) = bindings.get(&binding) else {
                    unreachable!(
                        "Parameter of branch not tracked in `bindings`."
                    );
                };

                local_types.get(type_).clone().into_type()
            }
            Pattern::Literal { .. } => Some(Type::Number),
        })
        .collect::<Option<_>>()?;

    let outputs = make_stack_direct(local_stack.get()?, &local_types)?;

    Some(Signature { inputs, outputs })
}

fn make_signature_indirect(
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

fn make_signature_direct(
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

fn make_stack_direct(
    local_stack: &[Index<InferredType>],
    local_types: &LocalTypes,
) -> Option<Vec<Type>> {
    local_stack
        .iter()
        .map(|index| local_types.get(index).clone().into_type())
        .collect::<Option<Vec<_>>>()
}

type Result<T> = result::Result<T, TypeError>;

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
    fn get(&self) -> Option<&Vec<Index<InferredType>>> {
        self.inner.as_ref()
    }

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

#[derive(Clone, Debug, Eq, PartialEq)]
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
