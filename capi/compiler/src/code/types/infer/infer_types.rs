use std::collections::BTreeMap;

use array_util::ArrayExt;

use crate::{
    code::{
        syntax::{
            Branch, Expression, FunctionLocation, Located, MemberLocation,
            Parameter, ParameterLocation, SyntaxTree,
        },
        types::repr::Stacks,
        Bindings, Dependencies, DependencyCluster, Environment,
        IdentifierTarget, Identifiers, Index, Signature, Type, TypeAnnotations,
    },
    intrinsics::IntrinsicFunction,
};

use super::{
    signature,
    types::{ExpectedType, InferredType, InferredTypes, Result, TypeError},
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
    pub bindings: BTreeMap<ParameterLocation, Type>,
    pub stacks: Stacks,
}

fn infer_cluster(
    cluster: &DependencyCluster,
    context: Context,
    output: &mut InferenceOutput,
) -> Result<()> {
    let mut cluster_functions = BTreeMap::new();
    let mut types = InferredTypes::default();

    for branch in cluster.branches(context.syntax_tree) {
        let function = (*branch.location.parent).clone();

        let environment = context.bindings.environment_of(&function);

        let (inputs, outputs) = infer_branch(
            branch,
            environment,
            &mut cluster_functions,
            &mut types,
            context,
            output,
        )?;

        if let Some(outputs) = outputs {
            let branch_signature = Signature { inputs, outputs };

            if let Some(function_signature) = cluster_functions.get(&function) {
                signature::unify(
                    [&branch_signature, function_signature],
                    &mut types,
                );
            }

            cluster_functions.insert(function.clone(), branch_signature);
        }
    }

    for (function, signature) in cluster_functions {
        if let Some(signature) = signature::make_direct(&signature, &types)? {
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

    let register_binding =
        |location: ParameterLocation, local_types: &mut InferredTypes| {
            let type_ = output
                .bindings
                .get(&location)
                .cloned()
                .map(InferredType::Known)
                .unwrap_or(InferredType::Unknown);
            let type_ = local_types.push(type_);
            (location, type_)
        };

    let parameters = branch
        .parameters()
        .map(|parameter| match parameter.fragment {
            Parameter::Binding(_) => {
                register_binding(parameter.location, local_types)
            }
            Parameter::Literal { .. } => {
                let type_ = local_types.push(InferredType::Known(Type::Number));
                (parameter.location, type_)
            }
        })
        .collect::<BTreeMap<_, _>>();

    let bindings = environment
        .bindings(context.syntax_tree)
        .map(|binding| register_binding(binding.location, local_types))
        .chain(parameters.clone())
        .collect::<BTreeMap<_, _>>();

    let inputs = parameters.into_values().collect();

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
        let Some(signature) = signature::make_direct(&signature, local_types)?
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

    let outputs = local_stack.get().cloned();

    Ok((inputs, outputs))
}

fn infer_expression(
    expression: Located<&Expression>,
    bindings: &BTreeMap<ParameterLocation, Index<InferredType>>,
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
        .map(|signature| signature::make_indirect(signature, local_types));

    let inferred = match expression.fragment {
        Expression::Identifier { name: identifier } => {
            match context.identifiers.is_resolved(&expression.location) {
                Some(target) => match target {
                    IdentifierTarget::Binding(binding) => {
                        let Some(output) = bindings.get(binding).copied()
                        else {
                            unreachable!(
                                "Identifier `{identifier}` has been resolved \
                                as binding, but it is not known in the branch. \
                                \n\
                                \n\
                                at {}\n\
                                \n\
                                binding at {}\n",
                                expression
                                    .location
                                    .display(context.syntax_tree),
                                binding.display(context.syntax_tree),
                            );
                        };
                        let signature = Signature {
                            inputs: vec![],
                            outputs: vec![output],
                        };
                        Some(signature)
                    }
                    IdentifierTarget::HostFunction(host) => {
                        let signature = signature::make_indirect(
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
                            signature::make_indirect(signature, local_types)
                        })
                    }
                    IdentifierTarget::UserDefinedFunction(user_defined) => {
                        functions
                            .get(user_defined)
                            .map(|signature| {
                                signature::make_indirect(
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
            let signature = signature::make_indirect(signature, local_types);
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
                signature::make_indirect(signature, local_types)
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
                    signature::make_direct(signature, local_types).transpose()
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
            let mut merge = |a: &Vec<Index<InferredType>>, b| {
                for (index_a, index_b) in a.iter().zip(b) {
                    local_types.unify([index_a, index_b]);
                }
            };

            merge(&inferred.inputs, &explicit.inputs);
            merge(&inferred.outputs, &explicit.outputs);

            Some(inferred)
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
