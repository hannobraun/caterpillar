use crate::{
    fragments::{
        Branch, Fragment, FragmentId, FragmentMap, Fragments, Function,
        Parameters,
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
        map: fragments,
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
        let terminator = Fragment::Terminator;
        let id = FragmentId::new(parent.as_ref(), None, &terminator);

        fragments.insert(id, terminator);

        id
    };

    for expression in expressions.into_iter().rev() {
        let fragment = compile_expression(expression, next, fragments);
        let id = FragmentId::new(parent.as_ref(), Some(&next), &fragment);

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
        let start = compile_context(branch.body, Some(next), fragments);

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

fn compile_expression(
    expression: syntax::Expression,
    next: FragmentId,
    fragments: &mut FragmentMap,
) -> Fragment {
    match expression {
        syntax::Expression::Comment { text } => Fragment::Comment { text },
        syntax::Expression::Function { function } => {
            compile_function(function, next, fragments)
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
                Some(IdentifierTarget::Function) => Fragment::CallToFunction {
                    name,
                    is_tail_call: is_in_tail_position,
                },
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
