use crate::{
    fragments::{
        Branch, Fragment, FragmentId, FragmentKind, FragmentMap, Fragments,
        Function, Hash, Parameters,
    },
    syntax::{self, IdentifierTarget},
};

pub fn generate_fragments(functions: Vec<syntax::Function>) -> Fragments {
    let mut fragments = FragmentMap::default();

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
    parent: Option<Hash<Fragment>>,
    fragments: &mut FragmentMap,
) -> Hash<Fragment>
where
    E: IntoIterator<Item = syntax::Expression>,
    E::IntoIter: DoubleEndedIterator,
{
    let mut next = {
        let terminator = Fragment {
            parent,
            next: None,
            kind: FragmentKind::Terminator,
        };
        let id = FragmentId {
            parent,
            next: None,
            here: terminator.hash(),
        };

        fragments.insert(id, terminator);

        id
    };

    for expression in expressions.into_iter().rev() {
        let fragment =
            compile_expression(expression, parent, next.here, fragments);
        let id = FragmentId {
            parent: fragment.parent,
            next: Some(next.hash()),
            here: fragment.hash(),
        };

        fragments.insert(id, fragment);

        next = id;
    }

    next.here
}

fn compile_function(
    function: syntax::Function,
    parent: Option<Hash<Fragment>>,
    next: Hash<Fragment>,
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
        next: Some(next),
        kind: FragmentKind::Function {
            function: Function {
                name: function.name,
                branches,
                environment: function.environment,
            },
        },
    }
}

fn compile_expression(
    expression: syntax::Expression,
    parent: Option<Hash<Fragment>>,
    next: Hash<Fragment>,
    fragments: &mut FragmentMap,
) -> Fragment {
    let fragment = match expression {
        syntax::Expression::Comment { text } => FragmentKind::Comment { text },
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
                    FragmentKind::ResolvedBinding { name }
                }
                Some(IdentifierTarget::Function) => {
                    FragmentKind::CallToFunction {
                        name,
                        is_tail_call: is_in_tail_position,
                    }
                }
                Some(IdentifierTarget::HostFunction { effect_number }) => {
                    FragmentKind::CallToHostFunction { effect_number }
                }
                Some(IdentifierTarget::Intrinsic { intrinsic }) => {
                    FragmentKind::CallToIntrinsic {
                        intrinsic,
                        is_tail_call: is_in_tail_position,
                    }
                }
                None => FragmentKind::UnresolvedIdentifier { name },
            }
        }
        syntax::Expression::Value(value) => FragmentKind::Value(value),
    };

    Fragment {
        parent,
        next: Some(next),
        kind: fragment,
    }
}

#[cfg(test)]
mod tests {
    use capi_runtime::Value;

    use crate::{
        fragments::{Fragment, FragmentKind, Fragments, Function},
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
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            kind:
                FragmentKind::Function {
                    function: Function { mut branches, .. },
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let branch = branches.remove(0);
        let body = fragments
            .iter_from(branch.start)
            .map(|fragment| fragment.kind.clone())
            .collect::<Vec<_>>();

        assert_eq!(
            body,
            [
                FragmentKind::Value(Value(1i32.to_le_bytes())),
                FragmentKind::Value(Value(1i32.to_le_bytes())),
                FragmentKind::Terminator,
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
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            kind:
                FragmentKind::Function {
                    function: Function { mut branches, .. },
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let branch = branches.remove(0);
        let last_fragment = fragments.iter_from(branch.start).last().unwrap();
        assert_eq!(last_fragment.kind, FragmentKind::Terminator);
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
            .remove(&fragments.root)
            .expect("Defined code, so there must be a root element.");
        let Fragment {
            next,
            kind:
                FragmentKind::Function {
                    function: Function { mut branches, .. },
                },
            ..
        } = root
        else {
            unreachable!("`f` must be the root element.");
        };
        let branch = branches.remove(0);
        let branch_fragments =
            fragments.iter_from(branch.start).collect::<Vec<_>>();
        let block_fragments = {
            let Fragment {
                kind: FragmentKind::Function { function },
                ..
            } = branch_fragments[0]
            else {
                panic!("Expected block")
            };

            let branch = function.branches.first().unwrap();

            fragments.iter_from(branch.start).collect::<Vec<_>>()
        };

        assert_eq!(branch_fragments[0].parent, next);
        assert_eq!(block_fragments[0].parent, Some(branch_fragments[1].hash()));
    }

    fn generate_fragments(functions: Vec<syntax::Function>) -> Fragments {
        super::generate_fragments(functions)
    }
}
