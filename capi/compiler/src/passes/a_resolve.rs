use std::collections::BTreeSet;

use capi_process::{builtin, Host};

use crate::repr::syntax::{Expression, ReferenceKind, Script};

pub fn resolve_references<H: Host>(script: &mut Script) {
    let mut bindings = BTreeSet::new();
    let user_functions = script
        .functions
        .iter()
        .map(|function| function.name.clone())
        .collect();

    for function in &mut script.functions {
        resolve_block::<H>(&mut function.body, &mut bindings, &user_functions);
    }
}

fn resolve_block<H: Host>(
    body: &mut [Expression],
    bindings: &mut BTreeSet<String>,
    user_functions: &BTreeSet<String>,
) {
    for expression in body {
        match expression {
            Expression::Binding { names } => {
                for name in names {
                    bindings.insert(name.clone());
                }
            }
            Expression::Block { body } => {
                resolve_block::<H>(body, bindings, user_functions)
            }
            Expression::Reference { name, kind } => {
                // The way this is written, definitions can silently shadow each
                // other in a defined order. This is undesirable.
                //
                // There should at least be a warning, if such shadowing
                // shouldn't be forbidden outright.
                if builtin(name).is_some()
                    || name == "return_if_non_zero"
                    || name == "return_if_zero"
                {
                    *kind = Some(ReferenceKind::BuiltinFunction);
                } else if H::function(name).is_some() {
                    *kind = Some(ReferenceKind::HostFunction);
                } else if user_functions.contains(name) {
                    *kind = Some(ReferenceKind::UserFunction);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use capi_process::{Effect, Host, HostFunction, Stack};

    use crate::repr::syntax::{Expression, ReferenceKind, Script};

    #[test]
    fn resolve_binding() {
        // Bindings can not be resolved yet by this pass, as it lacks the
        // information to do so. This is currently done by the fragment pass.

        let mut script = Script::default();
        script.function("f", [], |s| {
            s.v(0).bind(["value"]).r("value");
        });

        resolve_references(&mut script);

        assert_eq!(
            script.functions.remove(0).body.last(),
            Some(&Expression::Reference {
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
            Some(&Expression::Reference {
                name: String::from("brk"),
                kind: Some(ReferenceKind::BuiltinFunction),
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
            Some(&Expression::Reference {
                name: String::from("host_fn"),
                kind: Some(ReferenceKind::HostFunction),
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
            Some(&Expression::Reference {
                name: String::from("user_fn"),
                kind: Some(ReferenceKind::UserFunction),
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
