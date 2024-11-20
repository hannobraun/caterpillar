use std::collections::BTreeSet;

use crate::{
    code::{
        AnonymousFunctions, Branch, Expression, Function, FunctionLocation,
        Functions, Located,
    },
    host::Host,
    intrinsics::IntrinsicFunction,
};

/// # Resolve all identifiers, except those referring to user-defined functions
///
/// Identifiers referring to user-defined functions are identified as such, but
/// can not be resolved without a call graph. But by identifying them as such,
/// this compiler pass creates the prerequisite for creating a call graph.
pub fn resolve_most_identifiers(functions: &mut Functions, host: &impl Host) {
    let known_named_functions = functions
        .named
        .iter()
        .map(|function| function.name.clone())
        .collect();

    for function in functions.named.iter_mut() {
        resolve_in_function(
            function.into_located_function_mut(),
            &mut functions.anonymous,
            &known_named_functions,
            host,
        );
    }
}

fn resolve_in_function(
    function: Located<&mut Function>,
    anonymous_functions: &mut AnonymousFunctions,
    known_named_functions: &BTreeSet<String>,
    host: &impl Host,
) {
    let (branches, _) = function.destructure();

    for branch in branches {
        resolve_in_branch(
            branch,
            anonymous_functions,
            known_named_functions,
            host,
        );
    }
}

fn resolve_in_branch(
    branch: Located<&mut Branch>,
    anonymous_functions: &mut AnonymousFunctions,
    known_named_functions: &BTreeSet<String>,
    host: &impl Host,
) {
    let (body, _) = branch.destructure();

    for expression in body {
        match expression.fragment {
            Expression::UnresolvedIdentifier {
                name,
                is_known_to_be_call_to_user_defined_function,
            } => {
                // The way this is written, definitions can silently shadow each
                // other in a defined order. This is undesirable.
                //
                // There should at least be a warning, if such shadowing
                // shouldn't be forbidden outright.
                if let Some(intrinsic) = IntrinsicFunction::from_name(name) {
                    *expression.fragment =
                        Expression::CallToIntrinsicFunction { intrinsic };
                } else if let Some(function) = host.function_by_name(name) {
                    *expression.fragment = Expression::CallToHostFunction {
                        number: function.number(),
                    }
                } else if known_named_functions.contains(name) {
                    *is_known_to_be_call_to_user_defined_function = true;
                }
            }
            Expression::UnresolvedLocalFunction => {
                // Need to remove this and put it back below, since we need to
                // mutably borrow in between.
                let mut function =
                    anonymous_functions.remove(&expression.location).expect(
                        "The current expression is an anonymous function, or \
                        we wouldn't be in this `match` arm. It must be tracked \
                        in `anonymous_functions` under this location.\n\
                        \n\
                        Since this compiler pass is going through the code \
                        lexically, and locations are unique, we should not \
                        encounter this location again. Hence, that we remove \
                        an anonymous function from `anonymous_functions` here \
                        should have no bearing.",
                    );

                resolve_in_function(
                    Located {
                        fragment: &mut function,
                        location: FunctionLocation::AnonymousFunction {
                            location: expression.location.clone(),
                        },
                    },
                    anonymous_functions,
                    known_named_functions,
                    host,
                );

                anonymous_functions.insert(expression.location, function);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code::{syntax::parse, Branch, ConcreteSignature, Expression, Tokens},
        host::{Host, HostFunction},
        intrinsics::IntrinsicFunction,
    };

    #[test]
    fn resolve_host_function() {
        // The host can be queried to determine the existence of host functions.
        // We set up a special test host below, that provides the function that
        // is referenced here.

        let mut functions = resolve_identifiers(
            r"
                f: fn
                    \ ->
                        host_fn
                end
            ",
        );

        assert_eq!(
            functions
                .remove(0)
                .body
                .last_key_value()
                .map(|(_, expression)| expression),
            Some(&Expression::CallToHostFunction { number: 0 })
        );
    }

    #[test]
    fn resolve_intrinsic() {
        // Compiler intrinsics are special functions that aren't defined by the
        // host or user, but the compiler. They are translated into a series of
        // instructions at compile-time.

        let mut functions = resolve_identifiers(
            r"
                f: fn
                    \ ->
                        eval
                end
            ",
        );

        assert_eq!(
            functions
                .remove(0)
                .body
                .last_key_value()
                .map(|(_, expression)| expression),
            Some(&Expression::CallToIntrinsicFunction {
                intrinsic: IntrinsicFunction::Eval,
            })
        );
    }

    #[test]
    fn resolve_user_function() {
        // User-defined functions can be resolved by checking for the existence
        // of a matching function in the code.

        let mut functions = resolve_identifiers(
            r"
                f: fn
                    \ ->
                        user_fn
                end

                user_fn: fn
                    \ ->
                end
            ",
        );

        assert_eq!(
            functions
                .remove(0)
                .body
                .last_key_value()
                .map(|(_, expression)| expression),
            Some(&Expression::UnresolvedIdentifier {
                name: String::from("user_fn"),
                is_known_to_be_call_to_user_defined_function: true,
            })
        );
    }

    fn resolve_identifiers(input: &str) -> Vec<Branch> {
        let tokens = Tokens::from_input(input);
        let mut functions = parse(tokens);
        super::resolve_most_identifiers(&mut functions, &TestHost);

        functions
            .named
            .into_iter()
            .flat_map(|function| function.fragment.inner.branches.into_values())
            .collect()
    }

    struct TestHost;

    impl Host for TestHost {
        fn functions(&self) -> impl IntoIterator<Item = &dyn HostFunction> {
            [&TestFunction as &_]
        }
    }

    struct TestFunction;

    impl HostFunction for TestFunction {
        fn number(&self) -> u8 {
            0
        }

        fn name(&self) -> &'static str {
            "host_fn"
        }

        fn signature(&self) -> ConcreteSignature {
            ([], []).into()
        }
    }
}
