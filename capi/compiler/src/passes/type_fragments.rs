use std::{collections::BTreeMap, iter};

use crate::{
    code::{
        BranchLocation, CallGraph, Fragment, FragmentLocation, Function,
        FunctionLocation, NamedFunctions, Pattern, Signature, Type, Types,
    },
    intrinsics::IntrinsicFunction,
};

pub fn type_fragments(
    named_functions: &NamedFunctions,
    call_graph: &CallGraph,
) -> Types {
    let mut types = Types::default();

    for (index, _) in call_graph.functions_from_leaves() {
        let function = named_functions
            .find_by_index(index)
            .expect("Function referred to from call graph must exist.");

        type_fragments_in_function(
            &function.find,
            function.metadata,
            &mut types,
        );
    }

    types
}

fn type_fragments_in_function(
    function: &Function,
    location: FunctionLocation,
    types: &mut Types,
) {
    for (&index, branch) in function.branches.iter() {
        let location = BranchLocation {
            parent: Box::new(location.clone()),
            index,
        };

        let bindings = branch
            .parameters
            .iter()
            .filter_map(|pattern| match pattern {
                Pattern::Identifier { name } => Some(name),
                Pattern::Literal { .. } => None,
            })
            .zip(iter::from_fn(|| Some(types.inner.push(Type::Unknown))))
            .collect::<BTreeMap<_, _>>();
        let mut stack = Vec::new();

        for (&index, fragment) in branch.body.iter() {
            let location = FragmentLocation {
                parent: Box::new(location.clone()),
                index,
            };

            assert!(
                !types.for_fragments.contains_key(&location),
                "Encountered a fragment whose type signature has already been \
                inferred. But this is the first compiler pass that should do \
                so."
            );

            let signature = match fragment {
                Fragment::Binding { name, .. } => {
                    let Some(type_) = bindings.get(name).copied() else {
                        unreachable!(
                            "Can't find binding `{name}` in `bindings`, but \n\
                            \n\
                            a) all local bindings are added to `bindings` \
                               above, and\n\
                            b) if we encounter a `Fragment::Binding`, as we \
                               are here, that was put there by an earlier \
                               compiler pass _because_ it resolves to a local \
                               binding."
                        );
                    };

                    Some(Signature {
                        inputs: vec![],
                        outputs: vec![type_],
                    })
                }
                Fragment::CallToHostFunction { effect_number: _ } => {
                    // Not supported by inference yet.
                    None
                }
                Fragment::CallToIntrinsicFunction { intrinsic, .. } => {
                    match (intrinsic, intrinsic.signature()) {
                        (_, Some([inputs, outputs])) => {
                            let mut signature = Signature {
                                inputs: Vec::new(),
                                outputs: Vec::new(),
                            };

                            for input in inputs.iter().rev() {
                                if let Some(index) = stack.pop() {
                                    let type_ =
                                        types.inner.get_mut(&index).expect(
                                            "Type that is referenced from \
                                            stack must exist.",
                                        );
                                    *type_ = input.clone();
                                } else {
                                    // It looks like we don't have enough types
                                    // on the stack for the number of inputs
                                    // this fragment has.
                                    //
                                    // Typically, this would be an error in type
                                    // checking, but since the inference is not
                                    // fully implemented yet, it could also be
                                    // due to that.
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
                                stack.push(index);
                            }

                            types
                                .for_fragments
                                .insert(location.clone(), signature);

                            None
                        }
                        (IntrinsicFunction::Eval, None) => {
                            // Not supported by inference yet.
                            None
                        }
                        (intrinsic, signature) => {
                            unreachable!(
                                "Invalid combination of intrinsic \
                                (`{intrinsic:?}`) and signature \
                                (`{signature:?}`"
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
                Fragment::Function { function: _ } => {
                    // Not supported by inference yet.
                    None
                }
                Fragment::UnresolvedIdentifier { .. } => {
                    // There nothing we can do here, really. This has already
                    // been identified as a problem.
                    None
                }
                Fragment::Value(value) => {
                    // Not supported by inference yet.
                    let _ = value;
                    None
                }
            };

            if let Some(signature) = signature {
                for &output in &signature.outputs {
                    stack.push(output);
                }
                types.for_fragments.insert(location, signature);
            }
        }
    }
}
