use std::collections::BTreeMap;

use crate::{
    fragments::{
        Arguments, Cluster, Fragment, FragmentExpression, FragmentId,
        FragmentMap, FragmentPayload, Fragments, Function,
    },
    syntax::{Expression, IdentifierTarget},
};

use super::clusters;

pub fn generate_fragments(clusters: Vec<clusters::Cluster>) -> Fragments {
    let mut fragments = FragmentMap {
        inner: BTreeMap::new(),
    };

    let root = compile_context(
        clusters.into_iter().map(SyntaxElement::Item),
        None,
        &mut fragments,
    );

    Fragments {
        root,
        inner: fragments,
    }
}

fn compile_block(
    expressions: Vec<Expression>,
    parent: FragmentId,
    fragments: &mut FragmentMap,
) -> FragmentId {
    compile_context(
        expressions.into_iter().map(SyntaxElement::Expression),
        Some(parent),
        fragments,
    )
}

fn compile_context<E>(
    elements: E,
    parent: Option<FragmentId>,
    fragments: &mut FragmentMap,
) -> FragmentId
where
    E: IntoIterator<Item = SyntaxElement>,
    E::IntoIter: DoubleEndedIterator,
{
    let mut next = {
        let terminator = Fragment {
            parent,
            payload: FragmentPayload::Terminator,
        };
        let terminator_id = terminator.id();

        fragments.inner.insert(terminator_id, terminator);

        terminator_id
    };

    for element in elements.into_iter().rev() {
        let fragment = match element {
            SyntaxElement::Expression(expression) => {
                compile_expression(expression, parent, next, fragments)
            }
            SyntaxElement::Item(cluster) => {
                let mut members = Vec::new();

                for function in cluster.members {
                    let start = compile_block(function.body, next, fragments);

                    members.push(Function {
                        arguments: Arguments {
                            inner: function.arguments,
                        },
                        start,
                    });
                }

                Fragment {
                    parent,
                    payload: FragmentPayload::Cluster {
                        cluster: Cluster {
                            name: cluster.name,
                            members,
                        },
                        next,
                    },
                }
            }
        };

        next = fragment.id();

        fragments.inner.insert(fragment.id(), fragment);
    }

    next
}

fn compile_expression(
    expression: Expression,
    parent: Option<FragmentId>,
    next: FragmentId,
    fragments: &mut FragmentMap,
) -> Fragment {
    let expression = match expression {
        Expression::Binding { names } => {
            FragmentExpression::BindingDefinitions { names }
        }
        Expression::Block { body, environment } => {
            let start = compile_block(body, next, fragments);
            FragmentExpression::Block { start, environment }
        }
        Expression::Comment { text } => FragmentExpression::Comment { text },
        Expression::Identifier {
            name,
            target,
            is_known_to_be_in_tail_position,
        } => match target {
            Some(IdentifierTarget::Binding) => {
                FragmentExpression::ResolvedBinding { name }
            }
            Some(IdentifierTarget::BuiltinFunction) => {
                FragmentExpression::ResolvedBuiltinFunction { name }
            }
            Some(IdentifierTarget::Cluster) => {
                // By the time we make it to this compiler pass, all expressions
                // that are in tail position should be known to be so.
                let is_tail_call = is_known_to_be_in_tail_position;

                FragmentExpression::ResolvedCluster { name, is_tail_call }
            }
            Some(IdentifierTarget::HostFunction) => {
                FragmentExpression::ResolvedHostFunction { name }
            }
            None => FragmentExpression::UnresolvedIdentifier { name },
        },
        Expression::Value(value) => FragmentExpression::Value(value),
    };

    Fragment {
        parent,
        payload: FragmentPayload::Expression { expression, next },
    }
}

enum SyntaxElement {
    Expression(Expression),
    Item(clusters::Cluster),
}

#[cfg(test)]
mod tests {
    use capi_process::Value;

    use crate::{
        fragments::{
            Cluster, Fragment, FragmentExpression, FragmentPayload, Fragments,
        },
        passes::find_clusters,
        syntax::{self, Script},
    };

    #[test]
    fn duplicate_payload() {
        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.v(1).v(1);
            },
        );

        let mut fragments = generate_fragments(script.functions);

        let root = fragments
            .inner
            .inner
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            payload:
                FragmentPayload::Cluster {
                    cluster: Cluster { mut members, .. },
                    ..
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let function = members.remove(0);
        let body = fragments
            .inner
            .iter_from(function.start)
            .filter_map(|fragment| match &fragment.payload {
                FragmentPayload::Cluster { .. } => {
                    unreachable!(
                        "This test suite does not define functions within \
                        function bodies."
                    );
                }
                FragmentPayload::Expression { expression, .. } => {
                    Some(expression.clone())
                }
                FragmentPayload::Terminator => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            body,
            [
                FragmentExpression::Value(Value(1i32.to_le_bytes())),
                FragmentExpression::Value(Value(1i32.to_le_bytes())),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut script = Script::default();
        script.function("f", |p| p, |_| {});

        let mut fragments = generate_fragments(script.functions);

        let root = fragments
            .inner
            .inner
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            payload:
                FragmentPayload::Cluster {
                    cluster: Cluster { mut members, .. },
                    ..
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let function = members.remove(0);
        let last_fragment =
            fragments.inner.iter_from(function.start).last().unwrap();
        assert_eq!(last_fragment.payload, FragmentPayload::Terminator);
    }

    #[test]
    fn block_parent() {
        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.block(|_| {});
            },
        );

        let mut fragments = generate_fragments(script.functions);

        let root = fragments
            .inner
            .inner
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            payload:
                FragmentPayload::Cluster {
                    cluster: Cluster { mut members, .. },
                    next,
                    ..
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let function = members.remove(0);
        let function_fragments = fragments
            .inner
            .iter_from(function.start)
            .collect::<Vec<_>>();
        let block_fragments = {
            let Fragment {
                payload:
                    FragmentPayload::Expression {
                        expression: FragmentExpression::Block { start, .. },
                        ..
                    },
                ..
            } = function_fragments[0]
            else {
                panic!("Expected block")
            };

            fragments.inner.iter_from(*start).collect::<Vec<_>>()
        };

        assert_eq!(function_fragments[0].parent, Some(next));
        assert_eq!(block_fragments[0].parent, Some(function_fragments[1].id()));
    }

    fn generate_fragments(functions: Vec<syntax::Function>) -> Fragments {
        let clusters = find_clusters(functions);
        super::generate_fragments(clusters)
    }
}
