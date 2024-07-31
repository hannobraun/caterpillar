use std::collections::BTreeSet;

use capi_process::{builtin, Host};

use crate::repr::syntax::{Expression, IdentifierTarget, Script};

pub fn resolve_references<H: Host>(script: &mut Script) {
    let mut scopes = Scopes::new();
    let user_functions = script
        .functions
        .iter()
        .map(|function| function.name.clone())
        .collect();

    for function in &mut script.functions {
        scopes.push(function.args.clone().into_iter().collect());
        let mut environment = Environment::new();

        resolve_block::<H>(
            &mut function.body,
            &mut scopes,
            &mut environment,
            &user_functions,
        );

        assert!(
            environment.is_empty(),
            "Functions do not have an environment that they could access.",
        );
    }
}

fn resolve_block<H: Host>(
    body: &mut [Expression],
    scopes: &mut Scopes,
    environment: &mut Environment,
    user_functions: &BTreeSet<String>,
) {
    for expression in body {
        match expression {
            Expression::Binding { names } => {
                if let Some(bindings) = scopes.last_mut() {
                    for name in names {
                        bindings.insert(name.clone());
                    }
                }
            }
            Expression::Block { body, environment } => {
                scopes.push(Bindings::new());
                resolve_block::<H>(body, scopes, environment, user_functions);
            }
            Expression::Identifier { name, kind } => {
                // The way this is written, definitions can silently shadow each
                // other in a defined order. This is undesirable.
                //
                // There should at least be a warning, if such shadowing
                // shouldn't be forbidden outright.
                if scopes.iter().any(|bindings| bindings.contains(name)) {
                    *kind = Some(IdentifierTarget::Binding);

                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }
                }
                if builtin(name).is_some()
                    || name == "return_if_non_zero"
                    || name == "return_if_zero"
                {
                    *kind = Some(IdentifierTarget::BuiltinFunction);
                }
                if H::function(name).is_some() {
                    *kind = Some(IdentifierTarget::HostFunction);
                }
                if user_functions.contains(name) {
                    *kind = Some(IdentifierTarget::UserFunction);
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
    use capi_process::{Effect, Host, HostFunction, Stack};

    use crate::repr::syntax::{Expression, IdentifierTarget, Script};

    #[test]
    fn resolve_argument() {
        // Function arguments should be resolved within the function.

        let mut script = Script::default();
        script.function("f", ["argument"], |s| {
            s.r("argument");
        });

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("argument"),
                kind: Some(IdentifierTarget::Binding),
            })
        );
    }

    #[test]
    fn resolve_binding_from_same_scope() {
        // Bindings defined in the current scope should be resolved.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.v(0).bind(["value"]).r("value");
        });

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("value"),
                kind: Some(IdentifierTarget::Binding),
            })
        );
    }

    #[test]
    fn resolve_binding_from_parent_scope() {
        // Bindings defined in the lexical parent scope should be resolved in
        // the current scope.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.v(0).bind(["value"]).block(|s| {
                s.r("value");
            });
        });

        resolve_references(&mut script);

        let mut function = script.functions.remove(0);
        let Some(Expression::Block { body, environment }) =
            function.body.last_mut()
        else {
            panic!("Last expression in the function is a block.");
        };

        assert_eq!(
            body.last(),
            Some(&Expression::Identifier {
                name: String::from("value"),
                kind: Some(IdentifierTarget::Binding),
            })
        );

        assert!(environment.remove("value"));
        assert!(environment.is_empty());
    }

    #[test]
    fn do_not_resolve_binding_from_child_scope() {
        // Bindings that are defined in a scope that is a lexical child of the
        // current scope, should not be resolved.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.block(|s| {
                s.v(0).bind(["value"]);
            })
            .r("value");
        });

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("value"),
                kind: None,
            })
        );
    }

    #[test]
    fn resolve_builtin_function() {
        // Builtin functions are statically known, so any reference to one can
        // be determined without doubt.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.r("brk");
        });

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("brk"),
                kind: Some(IdentifierTarget::BuiltinFunction),
            })
        );
    }

    #[test]
    fn resolve_host_function() {
        // The host can be queried to determine the existence of host functions.
        // We set up a special test host below, that provides the function that
        // is referenced here.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.r("host_fn");
        });

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("host_fn"),
                kind: Some(IdentifierTarget::HostFunction),
            })
        );
    }

    #[test]
    fn resolve_user_function() {
        // User-defined functions can be resolved by checking for the existence
        // of a matching function in the code.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.r("user_fn");
        });
        script.function("user_fn", [], |_| {});

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("user_fn"),
                kind: Some(IdentifierTarget::UserFunction),
            })
        );
    }

    fn resolve_references(script: &mut Script) {
        super::resolve_references::<TestHost>(script)
    }

    struct TestHost {}

    impl Host for TestHost {
        type Effect = ();

        fn function(name: &str) -> Option<HostFunction<Self::Effect>> {
            match name {
                "host_fn" => Some(host_fn),
                _ => None,
            }
        }
    }

    fn host_fn(_: &mut Stack) -> Result<(), Effect<()>> {
        Ok(())
    }
}
