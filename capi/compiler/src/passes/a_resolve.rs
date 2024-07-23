use crate::repr::syntax::Script;

pub fn resolve_references(script: &mut Script) {
    let _ = script;
}

#[cfg(test)]
mod tests {
    use crate::repr::syntax::{Expression, Script};

    use super::resolve_references;

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
                kind: None
            })
        );
    }
}
