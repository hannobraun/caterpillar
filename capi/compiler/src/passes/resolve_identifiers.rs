use std::collections::BTreeSet;

use capi_process::{builtin, Host};

use crate::syntax::{Expression, IdentifierTarget, Pattern};

use super::clusters::Cluster;

pub fn resolve_identifiers<H: Host>(clusters: &mut Vec<Cluster>) {
    let mut scopes = Scopes::new();
    let known_clusters = clusters
        .iter()
        .map(|cluster| cluster.name.clone())
        .collect();

    for cluster in clusters {
        for function in &mut cluster.members {
            scopes.push(
                function
                    .arguments
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
            let mut environment = Environment::new();

            resolve_in_block::<H>(
                &mut function.body,
                &mut scopes,
                &mut environment,
                &known_clusters,
            );

            assert!(
                environment.is_empty(),
                "Functions do not have an environment that they could access.",
            );
        }
    }
}

fn resolve_in_block<H: Host>(
    body: &mut [Expression],
    scopes: &mut Scopes,
    environment: &mut Environment,
    known_clusters: &BTreeSet<String>,
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
                resolve_in_block::<H>(
                    body,
                    scopes,
                    environment,
                    known_clusters,
                );
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
                if builtin(name).is_some()
                    || name == "return_if_non_zero"
                    || name == "return_if_zero"
                {
                    *target = Some(IdentifierTarget::BuiltinFunction);
                }
                if H::function(name).is_some() {
                    *target = Some(IdentifierTarget::HostFunction);
                }
                if known_clusters.contains(name) {
                    *target = Some(IdentifierTarget::Cluster);
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
    use capi_process::Host;

    use crate::{
        passes::find_clusters,
        syntax::{Expression, Function, IdentifierTarget, Script},
    };

    #[test]
    fn resolve_argument() {
        // Function arguments should be resolved within the function.

        let mut script = Script::default();
        script.function(
            "f",
            |p| p.ident("argument"),
            |s| {
                s.ident("argument");
            },
        );

        let mut functions = resolve_identifiers(script);

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("argument"),
                target: Some(IdentifierTarget::Binding),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_binding_from_same_scope() {
        // Bindings defined in the current scope should be resolved.

        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.v(0).bind(["value"]).ident("value");
            },
        );

        let mut functions = resolve_identifiers(script);

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("value"),
                target: Some(IdentifierTarget::Binding),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_binding_from_parent_scope() {
        // Bindings defined in the lexical parent scope should be resolved in
        // the current scope.

        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.v(0).bind(["value"]).block(|s| {
                    s.ident("value");
                });
            },
        );

        let mut functions = resolve_identifiers(script);

        let mut function = functions.remove(0);
        let Some(Expression::Block { body, environment }) =
            function.body.last_mut()
        else {
            panic!("Last expression in the function is a block.");
        };

        assert_eq!(
            body.last(),
            Some(&Expression::Identifier {
                name: String::from("value"),
                target: Some(IdentifierTarget::Binding),
                is_known_to_be_in_tail_position: false,
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
        script.function(
            "f",
            |p| p,
            |s| {
                s.block(|s| {
                    s.v(0).bind(["value"]);
                })
                .ident("value");
            },
        );

        let mut functions = resolve_identifiers(script);

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
    fn resolve_builtin_function() {
        // Builtin functions are statically known, so any reference to one can
        // be determined without doubt.

        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.ident("brk");
            },
        );

        let mut functions = resolve_identifiers(script);

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("brk"),
                target: Some(IdentifierTarget::BuiltinFunction),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_host_function() {
        // The host can be queried to determine the existence of host functions.
        // We set up a special test host below, that provides the function that
        // is referenced here.

        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.ident("host_fn");
            },
        );

        let mut functions = resolve_identifiers(script);

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("host_fn"),
                target: Some(IdentifierTarget::HostFunction),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    #[test]
    fn resolve_user_function() {
        // User-defined functions can be resolved by checking for the existence
        // of a matching function in the code.

        let mut script = Script::default();
        script.function(
            "f",
            |p| p,
            |s| {
                s.ident("user_fn");
            },
        );
        script.function("user_fn", |p| p, |_| {});

        let mut functions = resolve_identifiers(script);

        assert_eq!(
            functions.remove(0).body.last(),
            Some(&Expression::Identifier {
                name: String::from("user_fn"),
                target: Some(IdentifierTarget::Cluster),
                is_known_to_be_in_tail_position: false,
            })
        );
    }

    fn resolve_identifiers(script: Script) -> Vec<Function> {
        let mut clusters = find_clusters(script.functions);
        super::resolve_identifiers::<TestHost>(&mut clusters);

        clusters
            .into_iter()
            .flat_map(|cluster| cluster.members)
            .collect()
    }

    struct TestHost {}

    impl Host for TestHost {
        type Effect = ();

        fn function(name: &str) -> Option<u8> {
            match name {
                "host_fn" => Some(0),
                _ => None,
            }
        }
    }
}
