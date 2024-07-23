use capi_process::{builtin, Host};

use crate::repr::syntax::{Expression, ReferenceKind, Script};

pub fn resolve_references<H: Host>(script: &mut Script) {
    for function in &mut script.functions {
        resolve_block::<H>(&mut function.body);
    }
}

fn resolve_block<H: Host>(body: &mut [Expression]) {
    for expression in body {
        if let Expression::Reference { name, kind } = expression {
            if builtin(name).is_some()
                || name == "return_if_non_zero"
                || name == "return_if_zero"
            {
                *kind = Some(ReferenceKind::BuiltinFunction);
            } else if H::function(name).is_some() {
                *kind = Some(ReferenceKind::HostFunction);
            }
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
