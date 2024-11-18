use std::collections::BTreeSet;

use crate::{
    code::{
        AnonymousFunctions, Branch, Expression, Function, FunctionLocation,
        Functions, Located, Pattern,
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
    let mut scopes = Scopes::new();
    let known_named_functions = functions
        .named
        .iter()
        .map(|function| function.name.clone())
        .collect();

    for function in functions.named.iter_mut() {
        resolve_in_function(
            function.into_located_function_mut(),
            &mut scopes,
            &mut functions.anonymous,
            &known_named_functions,
            host,
        );
    }
}

fn resolve_in_function(
    function: Located<&mut Function>,
    scopes: &mut Scopes,
    anonymous_functions: &mut AnonymousFunctions,
    known_named_functions: &BTreeSet<String>,
    host: &impl Host,
) {
    let (branches, environment) = function.destructure();

    for branch in branches {
        scopes.push(
            branch
                .parameters
                .clone()
                .into_iter()
                .filter_map(|pattern| match pattern {
                    Pattern::Identifier { name } => Some(name),
                    Pattern::Literal { .. } => {
                        // The scope is used to resolve identifiers against
                        // known bindings. Literal patterns don't create
                        // bindings, as their value is only used to select
                        // the function to be called.
                        None
                    }
                })
                .collect(),
        );

        resolve_in_branch(
            branch,
            scopes,
            environment,
            anonymous_functions,
            known_named_functions,
            host,
        );
    }
}

fn resolve_in_branch(
    branch: Located<&mut Branch>,
    scopes: &mut Scopes,
    environment: &mut Environment,
    anonymous_functions: &mut AnonymousFunctions,
    known_named_functions: &BTreeSet<String>,
    host: &impl Host,
) {
    let (body, parameters) = branch.destructure();

    for expression in body {
        match expression.fragment {
            Expression::UnresolvedIdentifier {
                name,
                is_known_to_be_in_tail_position,
                is_known_to_be_call_to_user_defined_function,
                ..
            } => {
                // The way this is written, definitions can silently shadow each
                // other in a defined order. This is undesirable.
                //
                // There should at least be a warning, if such shadowing
                // shouldn't be forbidden outright.
                if scopes.iter().any(|bindings| bindings.contains(name)) {
                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }

                    let mut index = 0;
                    for parameter in &*parameters {
                        match parameter {
                            Pattern::Identifier { name: n } => {
                                if n == name {
                                    break;
                                }
                            }
                            Pattern::Literal { .. } => {
                                continue;
                            }
                        }

                        index += 1;
                    }

                    *expression.fragment = Expression::Binding {
                        name: name.clone(),
                        index,
                    }
                } else if let Some(intrinsic) =
                    IntrinsicFunction::from_name(name)
                {
                    *expression.fragment =
                        Expression::CallToIntrinsicFunction {
                            intrinsic,
                            is_tail_call: *is_known_to_be_in_tail_position,
                        };
                } else if let Some(function) = host.function_by_name(name) {
                    *expression.fragment = Expression::CallToHostFunction {
                        number: function.number(),
                    }
                } else if known_named_functions.contains(name) {
                    *is_known_to_be_call_to_user_defined_function = true;
                }
            }
            Expression::UnresolvedLocalFunction { function, .. } => {
                resolve_in_function(
                    Located {
                        fragment: function,
                        location: FunctionLocation::AnonymousFunction {
                            location: expression.location.clone(),
                        },
                    },
                    scopes,
                    anonymous_functions,
                    known_named_functions,
                    host,
                );

                for name in &function.environment {
                    // If the child function we just resolved identifiers for
                    // captures something from its environment, and the current
                    // scope doesn't already have that, then it needs to capture
                    // it from its environment likewise.
                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }
                }

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
                    scopes,
                    anonymous_functions,
                    known_named_functions,
                    host,
                );

                for name in &function.environment {
                    // If the child function we just resolved identifiers for
                    // captures something from its environment, and the current
                    // scope doesn't already have that, then it needs to capture
                    // it from its environment likewise.
                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }
                }

                anonymous_functions.insert(expression.location, function);
            }
            _ => {}
        }
    }

    scopes.pop();
}

type Scopes = Vec<Bindings>;
type Bindings = BTreeSet<String>;
type Environment = BTreeSet<String>;

#[cfg(test)]
mod tests {
    use crate::{
        code::{Branch, ConcreteSignature, Expression},
        host::{Host, HostFunction},
        intrinsics::IntrinsicFunction,
        passes::{parse, tokenize},
    };

    #[test]
    fn do_not_resolve_binding_from_child_scope() {
        // Bindings that are defined in a scope that is a lexical child of the
        // current scope, should not be resolved.

        let mut functions = resolve_identifiers(
            r"
                f: fn
                    \ ->
                        0
                        fn
                            \ value ->
                        end
                        value
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
                name: String::from("value"),
                is_known_to_be_in_tail_position: false,
                is_known_to_be_call_to_user_defined_function: false,
            })
        );
    }

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
                is_tail_call: false
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
                is_known_to_be_in_tail_position: false,
                is_known_to_be_call_to_user_defined_function: true,
            })
        );
    }

    fn resolve_identifiers(source: &str) -> Vec<Branch> {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        super::resolve_most_identifiers(&mut functions, &TestHost {});

        functions
            .named
            .into_iter()
            .flat_map(|function| function.fragment.inner.branches.into_values())
            .collect()
    }

    struct TestHost {}

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
