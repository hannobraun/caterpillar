use std::{
    collections::{btree_map::Entry, BTreeMap, VecDeque},
    iter, vec,
};

use crate::{
    code::{
        Branch, BranchLocation, CallGraph, Cluster, ConcreteSignature,
        Fragment, FragmentLocation, FunctionLocation, Index, NamedFunctions,
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

    for cluster in call_graph.clusters_from_leaves() {
        infer_types_in_cluster(cluster, named_functions, host, &mut types);
    }

    types
}

fn infer_types_in_cluster(
    cluster: &Cluster,
    named_functions: &NamedFunctions,
    host: &impl Host,
    types: &mut Types,
) {
    let mut queue = VecDeque::new();

    for index in cluster.functions.values() {
        let function = named_functions
            .find_by_index(index)
            .expect("Function referred to from call graph must exist.");

        for (&index, branch) in function.branches.iter() {
            let location = BranchLocation {
                parent: Box::new(function.location().clone()),
                index,
            };

            let environment = BTreeMap::new();
            queue.push_back(QueueItem::new(
                branch,
                location,
                function.location(),
                &environment,
                types,
            ));
        }
    }

    infer_types_in_branches_of_cluster(
        queue,
        cluster,
        named_functions,
        host,
        types,
    );
}

fn infer_types_in_branches_of_cluster(
    mut queue: BranchQueue,
    cluster: &Cluster,
    named_functions: &NamedFunctions,
    host: &impl Host,
    types: &mut Types,
) {
    while let Some(queue_item) = queue.pop_front() {
        infer_types_in_branch(
            queue_item,
            cluster,
            named_functions,
            host,
            &mut queue,
            types,
        );
    }
}

fn infer_types_in_branch(
    mut queue_item: QueueItem,
    cluster: &Cluster,
    named_functions: &NamedFunctions,
    host: &impl Host,
    queue: &mut BranchQueue,
    types: &mut Types,
) {
    while let Some((index, fragment)) = queue_item.branch_body.peek() {
        let location = FragmentLocation {
            parent: Box::new(queue_item.branch_location.clone()),
            index: *index,
        };

        let inference = infer_type_of_fragment(
            fragment,
            &location,
            cluster,
            named_functions,
            &queue_item.bindings,
            host,
            queue,
            &mut queue_item.stack,
            types,
        );

        if let Some(inference) = inference {
            let FragmentInference::Inferred { signature } = inference;

            for &output in &signature.outputs {
                queue_item.stack.push(output);
            }
            types.for_fragments.insert(location, signature);
        }

        queue_item.branch_body.next().expect(
            "Just used `peek` to confirm there is an item in the queue; it \
            must still be there.",
        );
    }

    let signature = Signature {
        inputs: queue_item.parameters.into_iter().collect(),
        outputs: queue_item.stack,
    };

    types
        .for_branches
        .insert(queue_item.branch_location.clone(), signature.clone());

    types
        .for_branches
        .insert(queue_item.branch_location, signature.clone());

    match types.for_functions.entry(queue_item.function_location) {
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(signature);
        }
        Entry::Occupied(_occupied_entry) => {
            // If this isn't the first branch we're looking at, there
            // already is a function signature. We should compare that to
            // the new branch signature and make sure they're equal.
            //
            // As of this writing, type inference is only partially
            // implemented though, and as a result, this would trigger false
            // positives all the time.
            //
            // Let's just ignore any mismatches, for the time being.
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn infer_type_of_fragment(
    fragment: &Fragment,
    location: &FragmentLocation,
    cluster: &Cluster,
    named_functions: &NamedFunctions,
    bindings: &BTreeMap<String, Index<Type>>,
    host: &impl Host,
    queue: &mut BranchQueue,
    stack: &mut Vec<Index<Type>>,
    types: &mut Types,
) -> Option<FragmentInference> {
    assert!(
        !types.for_fragments.contains_key(location),
        "Encountered a fragment whose type signature has already been \
        inferred. But this is the first compiler pass that should do so."
    );

    let signature = match fragment {
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

            Signature {
                inputs: vec![],
                outputs: vec![type_],
            }
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
                    return None;
                }
                (intrinsic, signature) => {
                    unreachable!(
                        "Invalid combination of intrinsic (`{intrinsic:?}`) \
                        and signature (`{signature:?}`"
                    );
                }
            }
        }
        Fragment::CallToUserDefinedFunction { hash, .. } => {
            let function = named_functions
                .find_by_hash(hash)
                .expect("Function referred to by resolved call must exist.");

            types
                .for_functions
                .get(&function.location())
                .expect(
                    "This compiler pass infers function types by call graph, \
                    from the leaves up. Thus, the signature of a function must \
                    have been inferred, before a call for it is being \
                    inferred.",
                )
                .clone()
        }
        Fragment::CallToUserDefinedFunctionRecursive { index: _, .. } => {
            // Not supported by inference yet.
            return None;
        }
        Fragment::Comment { .. } => {
            // Comments have no bearing on type inference.
            return None;
        }
        Fragment::Function { function } => {
            let function_location = FunctionLocation::AnonymousFunction {
                location: location.clone(),
            };

            let signature = {
                for (&index, branch) in function.branches.iter() {
                    let branch_location = BranchLocation {
                        parent: Box::new(function_location.clone()),
                        index,
                    };

                    infer_types_in_branch(
                        QueueItem::new(
                            branch,
                            branch_location.clone(),
                            function_location.clone(),
                            bindings,
                            types,
                        ),
                        cluster,
                        named_functions,
                        host,
                        queue,
                        types,
                    );
                }

                if let Some(signature) =
                    types.for_functions.get(&function_location).cloned()
                {
                    let type_ = types.inner.push(Type::Function { signature });

                    Signature {
                        inputs: vec![],
                        outputs: vec![type_],
                    }
                } else {
                    unreachable!(
                        "Just inferred type of function; must be available."
                    );
                }
            };

            signature
        }
        Fragment::UnresolvedIdentifier { .. } => {
            // There nothing we can do here, really. This has already been
            // identified as a problem.
            return None;
        }
        Fragment::Value(_) => Signature {
            inputs: vec![],
            outputs: vec![types.inner.push(Type::Number)],
        },
    };

    Some(FragmentInference::Inferred { signature })
}

fn handle_concrete_signature(
    ConcreteSignature { inputs, outputs }: ConcreteSignature,
    stack: &mut Vec<Index<Type>>,
    types: &mut Types,
) -> Signature {
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

    signature
}

type BranchQueue<'r> = VecDeque<QueueItem>;

struct QueueItem {
    branch_body: iter::Peekable<vec::IntoIter<(Index<Fragment>, Fragment)>>,
    branch_location: BranchLocation,
    function_location: FunctionLocation,
    parameters: Vec<Index<Type>>,
    bindings: BTreeMap<String, Index<Type>>,
    stack: Vec<Index<Type>>,
}

impl QueueItem {
    fn new(
        branch: &Branch,
        branch_location: BranchLocation,
        function_location: FunctionLocation,
        environment: &BTreeMap<String, Index<Type>>,
        types: &mut Types,
    ) -> Self {
        let mut parameters = Vec::new();
        let mut bindings = environment.clone();

        for pattern in branch.parameters.iter() {
            let type_ = match pattern {
                Pattern::Identifier { name } => {
                    let type_ = types.inner.push(Type::Unknown);
                    bindings.insert(name.clone(), type_);
                    type_
                }
                Pattern::Literal { .. } => types.inner.push(Type::Number),
            };

            parameters.push(type_);
        }

        Self {
            branch_body: branch
                .body
                .iter()
                .map(|(index, fragment)| (*index, fragment.clone()))
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
            branch_location,
            function_location,
            parameters,
            bindings,
            stack: Vec::new(),
        }
    }
}

enum FragmentInference {
    Inferred { signature: Signature },
}

#[cfg(test)]
mod tests {
    use crate::{
        code::{ConcreteSignature, NamedFunctions, Type, Types},
        host::{Host, HostFunction},
        passes::{build_call_graph, parse, resolve_most_identifiers, tokenize},
    };

    #[test]
    fn infer_fragment_signatures_based_on_host_function() {
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

    #[test]
    fn infer_type_of_function_literal() {
        let (named_functions, types) = type_fragments(
            r"
                f: fn
                    \ ->
                        fn
                            \ 0 ->
                                0
                        end
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

        let function = fragments.next().unwrap();

        assert_eq!(function.inputs, []);

        let [Type::Function { signature }] = &function.outputs.as_slice()
        else {
            panic!("Unexpected outputs: {:?}", function.outputs);
        };
        let signature = signature.to_concrete_signature(&types).unwrap();

        use Type::*;
        assert_eq!(signature, ConcreteSignature::from(([Number], [Number])));
    }

    #[test]
    #[should_panic] // known bug; not currently tracked in an issue
    fn infer_type_of_function_based_on_most_specific_branch() {
        let (named_functions_a, types_a) = type_fragments(
            r"
                f: fn
                    \ 0 ->
                        0

                    \ n ->
                        n
                end
            ",
        );
        let (named_functions_b, types_b) = type_fragments(
            r"
                f: fn
                    \ n ->
                        n

                    \ 0 ->
                        0
                end
            ",
        );

        check(&named_functions_a, &types_a);
        check(&named_functions_b, &types_b);

        fn check(named_functions: &NamedFunctions, types: &Types) {
            let f = named_functions
                .find_by_name("f")
                .map(|function| {
                    types
                        .for_functions
                        .get(&function.location())
                        .unwrap()
                        .to_concrete_signature(types)
                        .unwrap()
                })
                .unwrap();

            use Type::*;
            assert_eq!(f, ConcreteSignature::from(([Number], [Number])),);
        }
    }

    #[test]
    fn infer_branch_signature() {
        let (named_functions, types) = type_fragments(
            r"
                f: fn
                    \ a, b, 0 ->
                        a number_to_nothing
                        b
                end
            ",
        );

        let branch = named_functions
            .find_by_name("f")
            .unwrap()
            .find_single_branch()
            .map(|branch| {
                types
                    .for_branches
                    .get(branch.location())
                    .unwrap()
                    .to_concrete_signature(&types)
                    .unwrap()
            })
            .unwrap();

        use Type::*;
        assert_eq!(
            branch,
            ConcreteSignature::from(([Number, Unknown, Number], [Unknown]))
        );
    }

    fn type_fragments(source: &str) -> (NamedFunctions, Types) {
        let tokens = tokenize(source);
        let mut named_functions = parse(tokens);
        resolve_most_identifiers(&mut named_functions, &TestHost);
        let call_graph = build_call_graph(&named_functions);
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
