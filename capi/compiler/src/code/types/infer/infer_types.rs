use std::{collections::BTreeMap, fmt::Write};

use array_util::ArrayExt;
use itertools::Itertools;

use crate::{
    code::{
        syntax::{
            Branch, Expression, FunctionLocation, Located, MemberLocation,
            Pattern, SyntaxTree,
        },
        types::repr::Stacks,
        Binding, Bindings, Dependencies, DependencyCluster, Environment,
        IdentifierTarget, Identifiers, Index, Signature, Type, TypeAnnotations,
    },
    intrinsics::IntrinsicFunction,
};

use super::types::{
    ExpectedType, InferredType, InferredTypes, Result, TypeError,
};

pub fn infer_types(context: Context) -> InferenceOutput {
    let mut output = InferenceOutput::default();

    for cluster in context.dependencies.clusters() {
        if let Err(TypeError {
            expected,
            actual,
            location,
        }) = infer_cluster(cluster, context, &mut output)
        {
            let actual = actual
                .map(|type_| format!("`{type_}`"))
                .unwrap_or_else(|| "nothing".to_string());

            let location = location
                .map(|location| {
                    format!("at {}\n", location.display(context.syntax_tree))
                })
                .unwrap_or(String::new());

            panic!(
                "\n\
                Type error: expected {expected}, got {actual}\n\
                \n\
                {location}",
            );
        }
    }

    output
}

#[derive(Clone, Copy)]
pub struct Context<'r> {
    pub syntax_tree: &'r SyntaxTree,
    pub bindings: &'r Bindings,
    pub identifiers: &'r Identifiers,
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

fn infer_cluster(
    cluster: &DependencyCluster,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<()> {
    let mut cluster_functions = BTreeMap::new();
    let mut local_types = InferredTypes::default();
    let mut branch_signatures_by_function = BTreeMap::new();

    for branch in cluster.branches(context.syntax_tree) {
        let function = (*branch.location.parent).clone();

        let environment = context.bindings.environment_of(&function);

        match infer_branch(
            branch,
            environment,
            &mut cluster_functions,
            &mut local_types,
            context,
            output,
        ) {
            Ok((inputs, outputs)) => {
                if let Some(outputs) = outputs.clone() {
                    let branch_signature = Signature {
                        inputs: inputs.clone(),
                        outputs,
                    };

                    let merged_signature = if let Some(function_signature) =
                        cluster_functions.get(&function)
                    {
                        unify_lists_of_types(
                            vec![
                                function_signature.inputs.iter().copied(),
                                branch_signature.inputs.iter().copied(),
                            ],
                            &mut local_types,
                        );
                        unify_lists_of_types(
                            vec![
                                function_signature.outputs.iter().copied(),
                                branch_signature.outputs.iter().copied(),
                            ],
                            &mut local_types,
                        );

                        branch_signature
                    } else {
                        branch_signature
                    };

                    cluster_functions
                        .insert(function.clone(), merged_signature);
                }

                branch_signatures_by_function
                    .entry(function)
                    .or_insert_with(Vec::new)
                    .push((inputs, outputs));
            }
            Err(TypeError {
                expected,
                actual,
                location,
            }) => {
                return Err(TypeError {
                    expected,
                    actual,
                    location,
                });
            }
        }
    }

    for (function, signature) in cluster_functions {
        if let Some(signature) =
            make_signature_direct(&signature, &local_types).unwrap()
        {
            output.functions.insert(function, signature);
        }
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
fn infer_branch(
    branch: Located<&Branch>,
    environment: &Environment,
    cluster_functions: &mut ClusterFunctions,
    local_types: &mut InferredTypes,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<(Vec<Index<InferredType>>, Option<Vec<Index<InferredType>>>)> {
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
            cluster_functions,
            local_types,
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
        let Some(signature) = make_signature_direct(&signature, local_types)?
        else {
            continue;
        };

        if let Some(binding) = context.bindings.is_binding(&location) {
            assert_eq!(signature.inputs.len(), 0);
            assert_eq!(signature.outputs.len(), 1);

            let type_ = signature.outputs[0].clone();

            output.bindings.insert(binding.clone(), type_);
        }

        output.expressions.insert(location, signature);
    }
    for (location, local_stack) in stacks {
        let Some(local_stack) = make_stack_direct(&local_stack, local_types)?
        else {
            continue;
        };

        output.stacks.insert(location, local_stack);
    }

    let (inputs, outputs) =
        infer_branch_signature(branch, bindings, local_types, local_stack);

    Ok((inputs, outputs))
}

fn infer_expression(
    expression: Located<&Expression>,
    bindings: &BTreeMap<Binding, Index<InferredType>>,
    functions: &BTreeMap<FunctionLocation, Signature>,
    cluster_functions: &mut ClusterFunctions,
    local_types: &mut InferredTypes,
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
            match context.identifiers.is_resolved(&expression.location) {
                Some(target) => match target {
                    IdentifierTarget::Binding(binding) => {
                        let Some(output) = bindings.get(binding).copied()
                        else {
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
                                let type_ = local_types.resolve(type_)?;
                                write!(
                                    available_bindings,
                                    "- index `{identifier_index}` at {}: \
                                    {type_:?}",
                                    branch.display(context.syntax_tree),
                                )
                                .expect("Writing to `String` can not fail.");
                            }

                            unreachable!(
                                "Identifier `{name}` has been resolved as \
                                binding, but it is not known in the branch.\n\
                                \n\
                                at {}\n\
                                \n\
                                Binding: identifier index `{identifier_index}` \
                                at {}\n\
                                \n\
                                Available bindings in branch:\n\
                                {available_bindings}",
                                expression
                                    .location
                                    .display(context.syntax_tree),
                                branch.display(context.syntax_tree),
                            );
                        };
                        let signature = Signature {
                            inputs: vec![],
                            outputs: vec![output],
                        };
                        Some(signature)
                    }
                    IdentifierTarget::HostFunction(host) => {
                        let signature = make_signature_indirect(
                            host.signature.clone(),
                            local_types,
                        );
                        Some(signature)
                    }
                    IdentifierTarget::IntrinsicFunction(intrinsic) => {
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
                    IdentifierTarget::UserDefinedFunction(user_defined) => {
                        functions
                            .get(user_defined)
                            .map(|signature| {
                                make_signature_indirect(
                                    signature.clone(),
                                    local_types,
                                )
                            })
                            .or_else(|| {
                                cluster_functions.get(user_defined).cloned()
                            })
                    }
                },
                None => None,
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
        [explicit.as_ref(), inferred.as_ref()].try_map_ext(|signature| {
            signature
                .and_then(|signature| {
                    make_signature_direct(signature, local_types).transpose()
                })
                .transpose()
        })?
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

                for (index_a, index_b) in a.iter().zip(b) {
                    let a = local_types.resolve(index_a)?;
                    let b = local_types.resolve(index_b)?;

                    let index = match (a, b) {
                        (
                            InferredType::Known(inferred),
                            InferredType::Known(explicit),
                        ) => {
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
                        (
                            InferredType::Known(_),
                            InferredType::Unknown { .. },
                        ) => index_a,
                        (
                            InferredType::Unknown { .. },
                            InferredType::Known(_),
                        ) => index_b,
                        (
                            InferredType::Unknown { .. },
                            InferredType::Unknown { .. },
                        ) => index_a,
                    };

                    indices.push(*index);
                }

                Ok(indices)
            };

            Some(Signature {
                inputs: merge(&inferred.inputs, &explicit.inputs)?,
                outputs: merge(&inferred.outputs, &explicit.outputs)?,
            })
        }
        (inferred, explicit) => inferred.or(explicit),
    };

    if let Some(signature) = signature {
        if let Some(local_stack) = local_stack.get_mut() {
            for input in signature.inputs.iter().rev() {
                match local_stack.pop() {
                    Some(operand) => {
                        local_types.unify([&operand, input]);
                    }
                    None => {
                        let input = local_types.resolve(input)?;

                        return Err(TypeError {
                            expected: input.into_expected_type(),
                            actual: None,
                            location: Some(expression.location),
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
    local_types: &mut InferredTypes,
    local_stack: &mut LocalStack,
) -> Result<Option<Signature>> {
    let signature = match intrinsic {
        IntrinsicFunction::Drop => {
            let Some(local_stack) = local_stack.get_mut() else {
                return Ok(None);
            };

            let top_operand = local_stack
                .last()
                .map(|index| local_types.resolve(index))
                .transpose()?;

            match top_operand {
                Some(InferredType::Known(type_)) => Some(Signature {
                    inputs: vec![type_],
                    outputs: vec![],
                }),
                Some(InferredType::Unknown { .. }) => None,
                None => {
                    return Err(TypeError {
                        expected: ExpectedType::Unknown,
                        actual: None,
                        location: Some(location.clone()),
                    });
                }
            }
        }
        IntrinsicFunction::Eval => {
            let Some(local_stack) = local_stack.get_mut() else {
                return Ok(None);
            };

            let top_operand = local_stack
                .last()
                .map(|index| local_types.resolve(index))
                .transpose()?;

            match top_operand {
                Some(InferredType::Known(Type::Function { signature })) => {
                    let outputs = signature.outputs.clone();
                    let inputs = signature
                        .inputs
                        .clone()
                        .into_iter()
                        .chain([Type::Function { signature }])
                        .collect();

                    Some(Signature { inputs, outputs })
                }
                Some(InferredType::Known(actual)) => {
                    return Err(TypeError {
                        expected: ExpectedType::Function,
                        actual: Some(actual.clone()),
                        location: Some(location.clone()),
                    });
                }
                Some(InferredType::Unknown { .. }) => None,
                None => {
                    return Err(TypeError {
                        expected: ExpectedType::Function,
                        actual: None,
                        location: Some(location.clone()),
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
    local_types: &mut InferredTypes,
    local_stack: LocalStack,
) -> (Vec<Index<InferredType>>, Option<Vec<Index<InferredType>>>) {
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

                *type_
            }
            Pattern::Literal { .. } => {
                local_types.push(InferredType::Known(Type::Number))
            }
        })
        .collect();

    let outputs = local_stack.get().cloned();

    (inputs, outputs)
}

fn unify_lists_of_types(
    mut lists_of_types: Vec<impl Iterator<Item = Index<InferredType>>>,
    local_types: &mut InferredTypes,
) {
    loop {
        let mut current_types = Vec::new();

        for types in &mut lists_of_types {
            current_types.push(types.next());
        }

        if current_types.iter().all(|input| input.is_none()) {
            break;
        }

        let pairs = current_types
            .into_iter()
            .map(|type_| {
                let Some(type_) = type_ else {
                    panic!(
                        "Found function with branches that have different \
                        number of inputs."
                    );
                };

                type_
            })
            .tuple_windows();

        for (a, b) in pairs {
            local_types.unify([&a, &b]);
        }
    }
}

fn make_signature_indirect(
    signature: Signature,
    local_types: &mut InferredTypes,
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
    local_types: &InferredTypes,
) -> Result<Option<Signature<Type>>> {
    let try_map = |from: &Vec<Index<InferredType>>| {
        from.iter()
            .map(|index| Ok(local_types.resolve(index)?.into_type()))
            .collect::<Result<Option<_>>>()
    };

    let inputs = try_map(&signature.inputs)?;
    let outputs = try_map(&signature.outputs)?;

    let signature = inputs
        .zip(outputs)
        .map(|(inputs, outputs)| Signature { inputs, outputs });

    Ok(signature)
}

fn make_stack_direct(
    local_stack: &[Index<InferredType>],
    local_types: &InferredTypes,
) -> Result<Option<Vec<Type>>> {
    local_stack
        .iter()
        .map(|index| Ok(local_types.resolve(index)?.into_type()))
        .collect()
}

type ClusterFunctions =
    BTreeMap<FunctionLocation, Signature<Index<InferredType>>>;

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
