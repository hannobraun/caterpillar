use std::{collections::BTreeMap, iter};

use crate::{
    code::{
        Branch, BranchLocation, CallGraph, ConcreteSignature, Fragment,
        FragmentLocation, Function, FunctionLocation, Index, NamedFunctions,
        Pattern, Signature, Type, Types,
    },
    host::Host,
    intrinsics::IntrinsicFunction,
};

pub fn infer_types(
    named_functions: &NamedFunctions,
    call_graph: &CallGraph,
    host: &impl Host,
) -> Types {
    let mut types = Types::default();

    for (index, _) in call_graph.functions_from_leaves() {
        let function = named_functions
            .find_by_index(index)
            .expect("Function referred to from call graph must exist.");

        let environment = BTreeMap::new();
        let signature = infer_types_in_function(
            &function.find,
            function.location(),
            &environment,
            host,
            &mut types,
        );

        types.for_functions.insert(function.location(), signature);
    }

    types
}

fn infer_types_in_function(
    function: &Function,
    location: FunctionLocation,
    environment: &BTreeMap<&String, Index<Type>>,
    host: &impl Host,
    types: &mut Types,
) -> Signature {
    let mut function_signature = None;

    for (&index, branch) in function.branches.iter() {
        let location = BranchLocation {
            parent: Box::new(location.clone()),
            index,
        };

        let branch_signature =
            infer_types_in_branch(branch, &location, environment, host, types);

        types
            .for_branches
            .insert(location, branch_signature.clone());

        // If this isn't the first branch we're looking at, there already is a
        // function signature. We should compare that to the new branch
        // signature and make sure they're equal.
        //
        // As of this writing, type inference is only partially implemented
        // though, and as a result, this would trigger false positives all the
        // time.
        //
        // Let's just ignore any mismatches, for the time being.
        function_signature = Some(branch_signature);
    }

    function_signature.unwrap_or_default()
}

fn infer_types_in_branch(
    branch: &Branch,
    location: &BranchLocation,
    environment: &BTreeMap<&String, Index<Type>>,
    host: &impl Host,
    types: &mut Types,
) -> Signature {
    let local_bindings = branch
        .parameters
        .iter()
        .filter_map(|pattern| match pattern {
            Pattern::Identifier { name } => Some(name),
            Pattern::Literal { .. } => None,
        })
        .zip(iter::from_fn(|| Some(types.inner.push(Type::Unknown))))
        .collect::<BTreeMap<_, _>>();
    let all_bindings = {
        let mut all_bindings = local_bindings.clone();
        all_bindings.extend(environment.iter());
        all_bindings
    };

    let mut stack = Vec::new();

    for (&index, fragment) in branch.body.iter() {
        let location = FragmentLocation {
            parent: Box::new(location.clone()),
            index,
        };

        let signature = infer_type_of_fragment(
            fragment,
            &location,
            host,
            &all_bindings,
            &mut stack,
            types,
        );

        if let Some(signature) = signature {
            for &output in &signature.outputs {
                stack.push(output);
            }
            types.for_fragments.insert(location, signature);
        }
    }

    Signature {
        inputs: local_bindings.into_values().collect(),
        outputs: stack,
    }
}

fn infer_type_of_fragment(
    fragment: &Fragment,
    location: &FragmentLocation,
    host: &impl Host,
    bindings: &BTreeMap<&String, Index<Type>>,
    stack: &mut Vec<Index<Type>>,
    types: &mut Types,
) -> Option<Signature> {
    assert!(
        !types.for_fragments.contains_key(location),
        "Encountered a fragment whose type signature has already been \
        inferred. But this is the first compiler pass that should do so."
    );

    match fragment {
        Fragment::Binding { name, .. } => {
            let Some(type_) = bindings.get(name).copied() else {
                unreachable!(
                    "Can't find binding `{name}` in `bindings`, but \n\
                    \n\
                    a) all local bindings are added to `bindings` above, and \n\
                    b) if we encounter a `Fragment::Binding`, as we are here, \
                       that was put there by an earlier compiler pass \
                       _because_ it resolves to a local binding."
                );
            };

            Some(Signature {
                inputs: vec![],
                outputs: vec![type_],
            })
        }
        Fragment::CallToHostFunction { number } => {
            let signature = host
                .function_by_number(*number)
                .expect(
                    "Call to host function has already been resolved. Must \
                    refer to a host function.",
                )
                .signature();

            handle_concrete_signature(signature, stack, types)
        }
        Fragment::CallToIntrinsicFunction { intrinsic, .. } => {
            match (intrinsic, intrinsic.signature()) {
                (_, Some(signature)) => {
                    handle_concrete_signature(signature, stack, types)
                }
                (IntrinsicFunction::Eval, None) => {
                    // Not supported by inference yet.
                    None
                }
                (intrinsic, signature) => {
                    unreachable!(
                        "Invalid combination of intrinsic (`{intrinsic:?}`) \
                        and signature (`{signature:?}`"
                    );
                }
            }
        }
        Fragment::CallToUserDefinedFunction {
            hash: _,
            is_tail_call: _,
        } => {
            // Not supported by inference yet.
            None
        }
        Fragment::CallToUserDefinedFunctionRecursive {
            index: _,
            is_tail_call: _,
        } => {
            // Not supported by inference yet.
            None
        }
        Fragment::Comment { .. } => {
            // Comments have no bearing on type inference.
            None
        }
        Fragment::Function { function } => {
            let location = FunctionLocation::AnonymousFunction {
                location: location.clone(),
            };

            let signature = infer_types_in_function(
                function, location, bindings, host, types,
            );

            Some(signature)
        }
        Fragment::UnresolvedIdentifier { .. } => {
            // There nothing we can do here, really. This has already been
            // identified as a problem.
            None
        }
        Fragment::Value(value) => {
            // Not supported by inference yet.
            let _ = value;
            None
        }
    }
}

fn handle_concrete_signature(
    ConcreteSignature { inputs, outputs }: ConcreteSignature,
    stack: &mut Vec<Index<Type>>,
    types: &mut Types,
) -> Option<Signature> {
    let mut signature = Signature {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    for input in inputs.iter().rev() {
        if let Some(index) = stack.pop() {
            let type_ = types
                .inner
                .get_mut(&index)
                .expect("Type that is referenced from stack must exist.");
            *type_ = input.clone();
        } else {
            // It looks like we don't have enough types on the stack for the
            // number of inputs this fragment has.
            //
            // Typically, this would be an error in type checking, but since the
            // inference is not fully implemented yet, it could also be due to
            // that.
            //
            // Let's ignore it for now.
        }
    }

    for input in inputs {
        let index = types.inner.push(input.clone());
        signature.inputs.push(index);
    }
    for output in outputs {
        let index = types.inner.push(output.clone());
        signature.outputs.push(index);
    }

    Some(signature)
}

#[cfg(test)]
mod tests {
    use crate::{
        code::{ConcreteSignature, NamedFunctions, Type, Types},
        host::{Host, HostFunction},
        passes::{
            create_call_graph, parse, resolve_most_identifiers, tokenize,
        },
    };

    #[test]
    fn fail() {
        let (named_functions, types) = type_fragments(
            r"
                f: fn
                    \ n ->
                        n number_to_nothing
                end
            ",
        );

        let mut fragments = named_functions
            .find_by_name("f")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|fragment| {
                types
                    .for_fragments
                    .get(fragment.location())
                    .unwrap()
                    .to_concrete_signature(&types)
                    .unwrap()
            });

        let n = fragments.next().unwrap();
        let host_fn = fragments.next().unwrap();

        use Type::*;
        assert_eq!(n, ConcreteSignature::from(([], [Number])));
        assert_eq!(host_fn, ConcreteSignature::from(([Number], [])));
    }

    fn type_fragments(source: &str) -> (NamedFunctions, Types) {
        let tokens = tokenize(source);
        let mut named_functions = parse(tokens);
        resolve_most_identifiers(&mut named_functions, &TestHost);
        let call_graph = create_call_graph(&named_functions);
        let types =
            super::infer_types(&named_functions, &call_graph, &TestHost);

        (named_functions, types)
    }

    struct TestHost;

    impl Host for TestHost {
        fn functions(&self) -> impl IntoIterator<Item = &dyn HostFunction> {
            [&NumberToNothing as &_]
        }
    }

    struct NumberToNothing;

    impl HostFunction for NumberToNothing {
        fn number(&self) -> u8 {
            0
        }

        fn name(&self) -> &'static str {
            "number_to_nothing"
        }

        fn signature(&self) -> ConcreteSignature {
            ConcreteSignature {
                inputs: vec![Type::Number],
                outputs: vec![],
            }
        }
    }
}
