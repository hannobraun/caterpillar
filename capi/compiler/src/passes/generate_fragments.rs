use crate::{
    fragments::{
        Branch, Cluster, Fragment, FragmentId, FragmentMap, Fragments,
        Function, Parameters,
    },
    hash::{Hash, NextNeighbor, PrevNeighbor},
    syntax::{self, IdentifierTarget},
};

pub fn generate_fragments(clusters: syntax::Clusters) -> Fragments {
    let mut fragments = FragmentMap::default();

    let mut compiled_functions = clusters
        .clusters
        .iter()
        .flat_map(|cluster| cluster.functions.values())
        .map(|index| {
            let function = clusters.functions[index].clone();
            let fragment = compile_function(function, &mut fragments);
            (index, fragment)
        })
        .collect::<Vec<_>>();
    compiled_functions.sort_by_key(|(index, _)| *index);

    let mut function_ids = Vec::new();
    let root = address_context(
        compiled_functions
            .into_iter()
            .map(|(_, fragment)| fragment)
            .collect(),
        &mut function_ids,
        &mut fragments,
    );

    let mut compiled_clusters = Vec::new();

    for cluster in clusters.clusters {
        let mut compiled_cluster = Cluster {
            functions: Vec::new(),
        };

        for index in cluster.functions.into_values() {
            let index: usize = index
                .0
                .try_into()
                .expect("Expecting `usize` to be at least 32-bit.");
            let id = function_ids[index];
            compiled_cluster.functions.push(id);
        }

        compiled_clusters.push(compiled_cluster);
    }

    Fragments {
        root,
        clusters: compiled_clusters,
        map: fragments,
    }
}

fn compile_context<E>(
    expressions: E,
    fragments: &mut FragmentMap,
) -> Option<FragmentId>
where
    E: IntoIterator<Item = syntax::Expression>,
    E::IntoIter: DoubleEndedIterator,
{
    let context = expressions
        .into_iter()
        .map(|expression| compile_expression(expression, fragments))
        .collect::<Vec<_>>();

    address_context(context, &mut Vec::new(), fragments)
}

fn address_context(
    context: Vec<Fragment>,
    ids: &mut Vec<FragmentId>,
    fragments: &mut FragmentMap,
) -> Option<FragmentId> {
    for fragment in &context {
        ids.push(FragmentId {
            prev: None,
            next: None,
            content: Hash::new(fragment),
        });
    }

    let mut prev = None;

    for id in ids.iter_mut() {
        let prev_hash = prev.as_ref().map(Hash::new);

        id.prev = prev_hash;
        prev = Some(PrevNeighbor {
            ulterior_neighbor: prev_hash,
            content: id.content,
        });
    }

    let mut next = None;

    for id in ids.iter_mut().rev() {
        let next_hash = next.as_ref().map(Hash::new);

        id.next = next_hash;
        next = Some(NextNeighbor {
            ulterior_neighbor: next_hash,
            content: id.content,
        });
    }

    for (i, (fragment, id)) in context.iter().zip(&*ids).enumerate() {
        let previous = if i == 0 {
            None
        } else {
            ids.get(i - 1).copied()
        };
        let next = ids.get(i + 1).copied();

        fragments.insert(*id, fragment.clone(), previous, next);
    }

    ids.first().copied()
}

fn compile_expression(
    expression: syntax::Expression,
    fragments: &mut FragmentMap,
) -> Fragment {
    match expression {
        syntax::Expression::Comment { text } => Fragment::Comment { text },
        syntax::Expression::Function { function } => {
            compile_function(function, fragments)
        }
        syntax::Expression::Identifier {
            name,
            target,
            is_known_to_be_in_tail_position,
        } => {
            // By the time we make it to this compiler pass, all expressions
            // that are in tail position should be known to be so.
            let is_in_tail_position = is_known_to_be_in_tail_position;

            match target {
                Some(IdentifierTarget::Binding) => {
                    Fragment::ResolvedBinding { name }
                }
                Some(IdentifierTarget::Function {
                    is_known_to_be_recursive_call_to_index,
                }) => {
                    // By the time we make it to this compiler pass, all calls
                    // that are recursive should be known to be so.
                    let is_recursive_call_to_index =
                        is_known_to_be_recursive_call_to_index;

                    if let Some(index) = is_recursive_call_to_index {
                        Fragment::CallToFunctionRecursive {
                            index,
                            is_tail_call: is_in_tail_position,
                        }
                    } else {
                        Fragment::CallToFunction {
                            name,
                            is_tail_call: is_in_tail_position,
                        }
                    }
                }
                Some(IdentifierTarget::HostFunction { effect_number }) => {
                    Fragment::CallToHostFunction { effect_number }
                }
                Some(IdentifierTarget::Intrinsic { intrinsic }) => {
                    Fragment::CallToIntrinsic {
                        intrinsic,
                        is_tail_call: is_in_tail_position,
                    }
                }
                None => Fragment::UnresolvedIdentifier { name },
            }
        }
        syntax::Expression::Value(value) => Fragment::Value(value),
    }
}

fn compile_function(
    function: syntax::Function,
    fragments: &mut FragmentMap,
) -> Fragment {
    let mut branches = Vec::new();

    for branch in function.branches {
        let start = compile_context(branch.body, fragments);

        branches.push(Branch {
            parameters: Parameters {
                inner: branch.parameters,
            },
            start,
        });
    }

    Fragment::Function {
        function: Function {
            name: function.name,
            branches,
            environment: function.environment,
        },
    }
}
