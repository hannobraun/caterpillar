use std::collections::{BTreeMap, BTreeSet};

use crate::repr::{
    fragments::{FragmentMap, FragmentParent, Fragments, Function},
    syntax::Script,
};

use super::passes::{
    build_fragments::compile_block, build_scopes::process_function,
};

pub fn script_to_fragments(script: Script) -> Fragments {
    let mut functions = BTreeSet::new();

    for function in &script.functions {
        if functions.contains(&function.name) {
            panic!("Can't re-define existing function `{}`.", function.name);
        }

        functions.insert(function.name.clone());
    }

    let mut fragments = FragmentMap {
        inner: BTreeMap::new(),
    };
    let mut by_function = Vec::new();

    for function in script.functions {
        let mut scopes =
            process_function(function.args.clone(), &function.body);
        let (start, environment) = compile_block(
            function.body,
            FragmentParent::Function {
                name: function.name.clone(),
            },
            &functions,
            &mut scopes,
            &mut fragments,
        );

        assert!(
            environment.is_empty(),
            "Functions have no environment that they could access. \n\
            - Function: {}\n\
            - Environment: {environment:#?}",
            function.name,
        );

        by_function.push(Function {
            name: function.name,
            args: function.args,
            start,
        });
    }

    Fragments {
        inner: fragments,
        by_function,
    }
}

#[cfg(test)]
mod tests {
    use capi_process::Value;

    use crate::repr::{
        fragments::{
            Fragment, FragmentExpression, FragmentParent, FragmentPayload,
            Fragments,
        },
        syntax::Script,
    };

    use super::script_to_fragments;

    #[test]
    fn arg_eval() {
        let mut script = Script::default();
        script.function("f", ["a"], |s| {
            s.w("a");
        });

        let fragments = script_to_fragments(script);

        let body = body(fragments);
        assert_eq!(
            body,
            [FragmentExpression::ResolvedBinding {
                name: String::from("a")
            }]
        );
    }

    #[test]
    fn binding_eval() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.v(0).bind(["b"]).w("b");
        });

        let fragments = script_to_fragments(script);

        let last = body(fragments).last().cloned().unwrap();
        assert_eq!(
            last,
            FragmentExpression::ResolvedBinding {
                name: String::from("b")
            }
        );
    }

    #[test]
    fn builtin_call() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.w("builtin");
        });

        let fragments = script_to_fragments(script);

        let body = body(fragments);
        assert_eq!(
            body,
            [FragmentExpression::ResolvedBuiltinFunction {
                name: String::from("builtin")
            }]
        );
    }

    #[test]
    fn function_call() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.w("g");
        });
        script.function("g", [], |_| {});

        let fragments = script_to_fragments(script);

        let body = body(fragments);
        assert_eq!(
            body,
            [FragmentExpression::ResolvedUserFunction {
                name: String::from("g")
            }]
        );
    }

    #[test]
    fn duplicate_payload() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.v(1).v(1);
        });

        let fragments = script_to_fragments(script);

        let body = body(fragments);
        assert_eq!(
            body,
            [
                FragmentExpression::Value(Value(1i32.to_le_bytes())),
                FragmentExpression::Value(Value(1i32.to_le_bytes())),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut script = Script::default();
        script.function("f", [], |_| {});

        let mut fragments = script_to_fragments(script);

        let start = fragments.by_function.remove(0).start;
        let last_fragment = fragments.inner.iter_from(start).last().unwrap();
        assert_eq!(last_fragment.payload, FragmentPayload::Terminator);
    }

    #[test]
    fn block_parent() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.block(|_| {});
        });

        let mut fragments = script_to_fragments(script);

        let start = fragments.by_function.remove(0).start;
        let function_fragments =
            fragments.inner.iter_from(start).collect::<Vec<_>>();
        let block_fragments = {
            let Fragment {
                payload:
                    FragmentPayload::Expression {
                        expression: FragmentExpression::Block { start, .. },
                        ..
                    },
                ..
            } = function_fragments[0]
            else {
                panic!("Expected block")
            };

            fragments.inner.iter_from(*start).collect::<Vec<_>>()
        };

        assert_eq!(
            function_fragments[0].parent,
            FragmentParent::Function {
                name: String::from("f")
            },
        );
        assert_eq!(
            block_fragments[0].parent,
            FragmentParent::Fragment {
                id: function_fragments[1].id()
            },
        );
    }

    fn body(mut fragments: Fragments) -> Vec<FragmentExpression> {
        let mut body = Vec::new();

        let start = fragments.by_function.remove(0).start;

        body.extend(fragments.inner.iter_from(start).filter_map(|fragment| {
            match &fragment.payload {
                FragmentPayload::Expression { expression, .. } => {
                    Some(expression.clone())
                }
                FragmentPayload::Terminator => None,
            }
        }));

        body
    }
}
