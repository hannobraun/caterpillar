mod passes;

use std::collections::{BTreeMap, BTreeSet};

use passes::build_scopes::build_scopes;

use crate::repr::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentParent,
        FragmentPayload, Fragments, Function,
    },
    syntax::{Expression, Script},
};

use self::passes::build_scopes::Bindings;

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
        let start = compile_function(
            function.name.clone(),
            function.args.clone(),
            function.body,
            &functions,
            &mut fragments,
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

fn compile_function(
    name: String,
    args: Vec<String>,
    body: Vec<Expression>,
    functions: &BTreeSet<String>,
    fragments: &mut FragmentMap,
) -> FragmentId {
    let bindings = build_scopes(args, &body);

    let expressions = body;

    compile_block(
        expressions,
        FragmentParent::Function { name },
        &bindings,
        functions,
        fragments,
    )
}

fn compile_block(
    expressions: Vec<Expression>,
    parent: FragmentParent,
    bindings: &Bindings,
    functions: &BTreeSet<String>,
    fragments: &mut FragmentMap,
) -> FragmentId {
    let mut next = {
        let terminator = Fragment {
            parent: parent.clone(),
            payload: FragmentPayload::Terminator,
        };
        let terminator_id = terminator.id();

        fragments.inner.insert(terminator_id, terminator);

        terminator_id
    };

    for expression in expressions.into_iter().rev() {
        let fragment = compile_expression(
            expression,
            parent.clone(),
            next,
            bindings,
            functions,
            fragments,
        );

        next = fragment.id();

        fragments.inner.insert(fragment.id(), fragment);
    }

    next
}

fn compile_expression(
    expression: Expression,
    parent: FragmentParent,
    next: FragmentId,
    bindings: &Bindings,
    functions: &BTreeSet<String>,
    fragments: &mut FragmentMap,
) -> Fragment {
    let expression = match expression {
        Expression::Binding { names } => {
            FragmentExpression::BindingDefinitions { names }
        }
        Expression::Block { expressions } => {
            let start = compile_block(
                expressions,
                FragmentParent::Fragment { id: next },
                bindings,
                functions,
                fragments,
            );
            FragmentExpression::Block { start }
        }
        Expression::Comment { text } => FragmentExpression::Comment { text },
        Expression::Value(value) => FragmentExpression::Value(value),
        Expression::Word { name } => {
            // The way this is written, bindings shadow built-in functions,
            // while user-defined functions shadow anything else.
            //
            // This isn't desirable. There should at least be a warning, if such
            // shadowing isn't forbidden outright. It'll do for now though.
            if functions.contains(&name) {
                FragmentExpression::FunctionCall { name }
            } else if bindings.inner.contains(&name) {
                FragmentExpression::BindingEvaluation { name }
            } else {
                // This doesn't check whether the built-in function exists, and
                // given how built-in functions are currently defined, that's
                // not practical to implement.
                FragmentExpression::BuiltinCall { name }
            }
        }
    };

    Fragment {
        parent,
        payload: FragmentPayload::Expression { expression, next },
    }
}

#[cfg(test)]
mod tests {
    use capi_process::Value;

    use crate::{
        repr::{
            fragments::{Fragment, FragmentParent, FragmentPayload, Fragments},
            syntax::Script,
        },
        stages::script_to_fragments::FragmentExpression,
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
            [FragmentExpression::BindingEvaluation {
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
            FragmentExpression::BindingEvaluation {
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
            [FragmentExpression::BuiltinCall {
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
            [FragmentExpression::FunctionCall {
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
                        expression: FragmentExpression::Block { start },
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
