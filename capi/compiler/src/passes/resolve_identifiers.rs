use std::collections::BTreeSet;

use crate::{
    host::Host,
    intrinsics::Intrinsic,
    syntax::{Branch, Expression, Function, IdentifierTarget, Pattern},
};

pub fn resolve_identifiers<H: Host>(functions: &mut Vec<Function>) {
    let mut scopes = Scopes::new();
    let known_named_functions = functions
        .iter()
        .filter_map(|function| function.name.clone())
        .collect();

    for function in functions {
        if !function.environment.is_empty() {
            panic!(
                "Named functions do not have an environment that they could \
                access.\n\
                \n\
                Environment: {:#?}",
                function.environment,
            );
        }

        resolve_in_function::<H>(
            &mut function.branches,
            &mut function.environment,
            &mut scopes,
            &known_named_functions,
        );
    }
}

fn resolve_in_function<H: Host>(
    branches: &mut Vec<Branch>,
    environment: &mut Environment,
    scopes: &mut Scopes,
    known_named_functions: &BTreeSet<String>,
) {
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

        resolve_in_branch::<H>(
            &mut branch.body,
            scopes,
            environment,
            known_named_functions,
        );
    }
}

fn resolve_in_branch<H: Host>(
    body: &mut [Expression],
    scopes: &mut Scopes,
    environment: &mut Environment,
    known_named_functions: &BTreeSet<String>,
) {
    for expression in body {
        match expression {
            Expression::Function { function } => {
                resolve_in_function::<H>(
                    &mut function.branches,
                    &mut function.environment,
                    scopes,
                    known_named_functions,
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
            }
            Expression::Identifier { name, target, .. } => {
                // The way this is written, definitions can silently shadow each
                // other in a defined order. This is undesirable.
                //
                // There should at least be a warning, if such shadowing
                // shouldn't be forbidden outright.
                if scopes.iter().any(|bindings| bindings.contains(name)) {
                    *target = Some(IdentifierTarget::Binding);

                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }
                }
                if let Some(intrinsic) = Intrinsic::from_name(name) {
                    *target = Some(IdentifierTarget::Intrinsic { intrinsic });
                }
                if let Some(effect_number) =
                    H::function_name_to_effect_number(name)
                {
                    *target =
                        Some(IdentifierTarget::HostFunction { effect_number });
                }
                if known_named_functions.contains(name) {
                    *target = Some(IdentifierTarget::Function {
                        is_known_to_be_recursive_call_to_index: None,
                    });
                }
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
        host::Host,
        intrinsics::Intrinsic,
        passes::{parse, tokenize},
        syntax::{Branch, Expression, Function, IdentifierTarget},
    };

    #[test]
    fn do_not_resolve_binding_from_child_scope() {
        // Bindings that are defined in a scope that is a lexical child of the
        // current scope, should not be resolved.

        let mut functions = resolve_identifiers(code(
            r"
                f: {
                    \ ->
                        0
                        {
                            \ value ->
                        }
                        value
                }
            ",
        ));

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("value"),
                target: None,
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_host_function() {
        // The host can be queried to determine the existence of host functions.
        // We set up a special test host below, that provides the function that
        // is referenced here.

        let mut functions = resolve_identifiers(code(
            r"
                f: {
                    \ ->
                        host_fn
                }
            ",
        ));

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("host_fn"),
                target: Some(IdentifierTarget::HostFunction {
                    effect_number: 0
                }),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_intrinsic() {
        // Compiler intrinsics are special functions that aren't defined by the
        // host or user, but the compiler. They are translated into a series of
        // instructions at compile-time.

        let mut functions = resolve_identifiers(code(
            r"
                f: {
                    \ ->
                        eval
                }
            ",
        ));

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("eval"),
                target: Some(IdentifierTarget::Intrinsic {
                    intrinsic: Intrinsic::Eval
                }),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_user_function() {
        // User-defined functions can be resolved by checking for the existence
        // of a matching function in the code.

        let mut functions = resolve_identifiers(code(
            r"
                f: {
                    \ ->
                        user_fn
                }

                user_fn: {
                    \ ->
                }
            ",
        ));

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("user_fn"),
                target: Some(IdentifierTarget::Function {
                    is_known_to_be_recursive_call_to_index: None
                }),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    fn resolve_identifiers(mut functions: Vec<Function>) -> Vec<Branch> {
        super::resolve_identifiers::<TestHost>(&mut functions);

        functions
            .into_iter()
            .flat_map(|function| function.branches)
            .collect()
    }

    struct TestHost {}

    impl Host for TestHost {
        fn effect_number_to_function_name(effect: u8) -> Option<&'static str> {
            match effect {
                0 => Some("host_fn"),
                _ => None,
            }
        }

        fn function_name_to_effect_number(name: &str) -> Option<u8> {
            match name {
                "host_fn" => Some(0),
                _ => None,
            }
        }
    }

    fn code(source: &str) -> Vec<Function> {
        let tokens = tokenize(source);
        parse(tokens)
    }
}
