use capi_process::builtin;

use crate::repr::syntax::{Expression, ReferenceKind, Script};

pub fn resolve_references(script: &mut Script) {
    for function in &mut script.functions {
        for expression in &mut function.body {
            if let Expression::Reference { name, kind } = expression {
                if builtin(name).is_some()
                    || name == "return_if_non_zero"
                    || name == "return_if_zero"
                {
                    *kind = Some(ReferenceKind::BuiltinFunction);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn resolve_builtin_functions() {
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

    fn resolve_references(script: &mut Script) {
        super::resolve_references(script)
    }
}
