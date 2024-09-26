use crate::{
    fragments::{
        Branch, Fragment, FragmentId, FragmentMap, Fragments, Function,
        Parameters,
    },
    hash::{Hash, NextNeighbor, PrevNeighbor},
    syntax::{self, IdentifierTarget},
};

pub fn generate_fragments(functions: Vec<syntax::Cluster>) -> Fragments {
    let mut fragments = FragmentMap::default();

    let root = compile_context(
        functions
            .into_iter()
            .flat_map(|cluster| cluster.functions)
            .map(|function| syntax::Expression::Function { function })
            .collect::<Vec<_>>(),
        &mut fragments,
    );

    Fragments {
        root,
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
    let mut new_fragments = expressions
        .into_iter()
        .map(|expression| {
            let fragment = compile_expression(expression, fragments);
            let id = FragmentId {
                prev: None,
                next: None,
                content: Hash::new(&fragment),
            };

            (fragment, id)
        })
        .collect::<Vec<_>>();

    let mut prev = None;

    for (_, id) in &mut new_fragments {
        let prev_hash = prev.as_ref().map(Hash::new);

        id.prev = prev_hash;
        prev = Some(PrevNeighbor {
            ulterior_neighbor: prev_hash,
            content: id.content,
        });
    }

    let mut start = None;
    let mut next = None;

    for (_, id) in new_fragments.iter_mut().rev() {
        let next_hash = next.as_ref().map(Hash::new);

        id.next = next_hash;
        next = Some(NextNeighbor {
            ulterior_neighbor: next_hash,
            content: id.content,
        });

        start = Some(*id);
    }

    for (i, (fragment, id)) in new_fragments.iter().enumerate() {
        let previous = if i == 0 {
            None
        } else {
            new_fragments.get(i - 1).map(|(_, id)| *id)
        };
        let next = new_fragments.get(i + 1).map(|(_, id)| *id);

        fragments.insert(*id, fragment.clone(), previous, next);
    }

    start
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
                    is_known_to_be_recursive,
                }) => {
                    // We ignore this right now. But once function calls refer
                    // to the called function by fragment ID, and no longer by
                    // name, we need this information.
                    let _ = is_known_to_be_recursive;

                    Fragment::CallToFunction {
                        name,
                        is_tail_call: is_in_tail_position,
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
