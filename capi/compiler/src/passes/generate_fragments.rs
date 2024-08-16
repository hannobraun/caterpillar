use std::collections::BTreeMap;

use crate::{
    fragments::{
        Branch, Expression, Fragment, FragmentId, FragmentMap, FragmentPayload,
        Fragments, Function, Parameters,
    },
    syntax::{self, IdentifierTarget},
};

pub fn generate_fragments(functions: Vec<syntax::Function>) -> Fragments {
    let mut fragments = FragmentMap {
        inner: BTreeMap::new(),
    };

    let root = compile_context(
        functions
            .into_iter()
            .map(|function| syntax::Expression::Function { function }),
        None,
        &mut fragments,
    );

    Fragments {
        root,
        inner: fragments,
    }
}

fn compile_context<E>(
    expressions: E,
    parent: Option<FragmentId>,
    fragments: &mut FragmentMap,
) -> FragmentId
where
    E: IntoIterator<Item = syntax::Expression>,
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

    for expression in expressions.into_iter().rev() {
        let fragment = compile_expression(expression, parent, next, fragments);

        next = fragment.id();

        fragments.inner.insert(fragment.id(), fragment);
    }

    next
}

fn compile_function(
    function: syntax::Function,
    parent: Option<FragmentId>,
    next: FragmentId,
    fragments: &mut FragmentMap,
) -> Fragment {
    let mut branches = Vec::new();

    for branch in function.branches {
        let start = compile_context(branch.body, Some(next), fragments);

        branches.push(Branch {
            parameters: Parameters {
                inner: branch.parameters,
            },
            start,
        });
    }

    Fragment {
        parent,
        payload: FragmentPayload::Expression {
            expression: Expression::Function {
                function: Function {
                    name: function.name,
                    branches,
                    environment: function.environment,
                },
            },
            next,
        },
    }
}

fn compile_expression(
    expression: syntax::Expression,
    parent: Option<FragmentId>,
    next: FragmentId,
    fragments: &mut FragmentMap,
) -> Fragment {
    let expression = match expression {
        syntax::Expression::Binding { names } => {
            Expression::BindingDefinitions { names }
        }
        syntax::Expression::Comment { text } => Expression::Comment { text },
        syntax::Expression::Function { function } => {
            return compile_function(function, parent, next, fragments);
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
                    Expression::ResolvedBinding { name }
                }
                Some(IdentifierTarget::BuiltinFunction) => {
                    Expression::ResolvedBuiltinFunction { name }
                }
                Some(IdentifierTarget::Function) => {
                    Expression::ResolvedFunction {
                        name,
                        is_tail_call: is_in_tail_position,
                    }
                }
                Some(IdentifierTarget::HostFunction) => {
                    Expression::ResolvedHostFunction { name }
                }
                None => Expression::UnresolvedIdentifier { name },
            }
        }
        syntax::Expression::Value(value) => Expression::Value(value),
    };

    Fragment {
        parent,
        payload: FragmentPayload::Expression { expression, next },
    }
}

#[cfg(test)]
mod tests {
    use capi_process::Value;

    use crate::{
        fragments::{
            Expression, Fragment, FragmentPayload, Fragments, Function,
        },
        syntax::{self, Script},
    };

    #[test]
    fn duplicate_payload() {
        let mut script = Script::default();
        script.function("f", |b| {
            b.branch(
                |p| p,
                |s| {
                    s.v(1).v(1);
                },
            )
        });

        let mut fragments = generate_fragments(script.functions);

        let root = fragments
            .inner
            .inner
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            payload:
                FragmentPayload::Expression {
                    expression:
                        Expression::Function {
                            function: Function { mut branches, .. },
                        },
                    ..
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let branch = branches.remove(0);
        let body = fragments
            .inner
            .iter_from(branch.start)
            .filter_map(|fragment| match &fragment.payload {
                FragmentPayload::Expression { expression, .. } => {
                    Some(expression.clone())
                }
                FragmentPayload::Terminator => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            body,
            [
                Expression::Value(Value(1i32.to_le_bytes())),
                Expression::Value(Value(1i32.to_le_bytes())),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut script = Script::default();
        script.function("f", |b| b.branch(|p| p, |_| {}));

        let mut fragments = generate_fragments(script.functions);

        let root = fragments
            .inner
            .inner
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            payload:
                FragmentPayload::Expression {
                    expression:
                        Expression::Function {
                            function: Function { mut branches, .. },
                        },
                    ..
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let branch = branches.remove(0);
        let last_fragment =
            fragments.inner.iter_from(branch.start).last().unwrap();
        assert_eq!(last_fragment.payload, FragmentPayload::Terminator);
    }

    #[test]
    fn block_parent() {
        let mut script = Script::default();
        script.function("f", |b| {
            b.branch(
                |p| p,
                |s| {
                    s.fun(|b| b.branch(|b| b, |_| {}));
                },
            )
        });

        let mut fragments = generate_fragments(script.functions);

        let root = fragments
            .inner
            .inner
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            payload:
                FragmentPayload::Expression {
                    expression:
                        Expression::Function {
                            function: Function { mut branches, .. },
                        },
                    next,
                    ..
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let branch = branches.remove(0);
        let branch_fragments =
            fragments.inner.iter_from(branch.start).collect::<Vec<_>>();
        let block_fragments = {
            let Fragment {
                payload:
                    FragmentPayload::Expression {
                        expression: Expression::Function { function },
                        ..
                    },
                ..
            } = branch_fragments[0]
            else {
                panic!("Expected block")
            };

            let branch = function.branches.first().unwrap();

            fragments.inner.iter_from(branch.start).collect::<Vec<_>>()
        };

        assert_eq!(branch_fragments[0].parent, Some(next));
        assert_eq!(block_fragments[0].parent, Some(branch_fragments[1].id()));
    }

    fn generate_fragments(functions: Vec<syntax::Function>) -> Fragments {
        super::generate_fragments(functions)
    }
}
