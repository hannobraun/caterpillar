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
    parent: Option<Hash<FragmentId>>,
    fragments: &mut FragmentMap,
) -> FragmentId
where
    E: IntoIterator<Item = syntax::Expression>,
    E::IntoIter: DoubleEndedIterator,
{
    let mut next = {
        let terminator = Fragment {
            kind: FragmentKind::Terminator,
        };
        let id = FragmentId {
            parent,
            next: None,
            this: terminator.hash(),
        };

        fragments.insert(id, terminator);

        id
    };

    for expression in expressions.into_iter().rev() {
        let fragment = compile_expression(expression, next, fragments);
        let id = FragmentId {
            parent,
            next: Some(next.hash()),
            this: fragment.hash(),
        };

        fragments.insert(id, fragment);

        next = id;
    }

    next
}

fn compile_function(
    function: syntax::Function,
    next: FragmentId,
    fragments: &mut FragmentMap,
) -> Fragment {
    let mut branches = Vec::new();

    for branch in function.branches {
        let start = compile_context(branch.body, Some(next.hash()), fragments);

        branches.push(Branch {
            parameters: Parameters {
                inner: branch.parameters,
            },
            start,
        });
    }

    Fragment {
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
    next: FragmentId,
    fragments: &mut FragmentMap,
) -> Fragment {
    let fragment = match expression {
        syntax::Expression::Comment { text } => FragmentKind::Comment { text },
        syntax::Expression::Function { function } => {
            return compile_function(function, next, fragments);
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

    Fragment { kind: fragment }
}
