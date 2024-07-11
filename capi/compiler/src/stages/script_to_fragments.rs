use std::collections::{BTreeMap, BTreeSet};

use crate::repr::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentParent,
        FragmentPayload, Fragments, Function,
    },
    syntax::{Expression, Script},
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
        let start = compile_function(
            function.name.clone(),
            &function.args,
            function.expressions,
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
    args: &[String],
    body: Vec<Expression>,
    functions: &BTreeSet<String>,
    fragments: &mut FragmentMap,
) -> FragmentId {
    let mut bindings: BTreeSet<_> = args.iter().cloned().collect();

    for expression in &body {
        if let Expression::Binding { names } = expression {
            for name in names.iter().cloned().rev() {
                // Inserting bindings unconditionally like this does mean that
                // bindings can overwrite previously defined bindings. This is
                // undesirable, but it'll do for now.
                bindings.insert(name);
            }
        }
    }

    let parent = FragmentParent::Function { name: name.clone() };
    let mut next = {
        let terminator = Fragment {
            parent: parent.clone(),
            next: None,
            payload: FragmentPayload::Terminator,
        };
        let terminator_id = terminator.id();

        fragments.inner.insert(terminator_id, terminator);

        terminator_id
    };

    for expression in body.into_iter().rev() {
        let fragment = compile_expression(
            expression,
            parent.clone(),
            next,
            &bindings,
            functions,
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
    bindings: &BTreeSet<String>,
    functions: &BTreeSet<String>,
) -> Fragment {
    let expression = match expression {
        Expression::Binding { names } => {
            FragmentExpression::BindingDefinitions { names }
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
            } else if bindings.contains(&name) {
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
        next: Some(next),
        payload: FragmentPayload::Expression { expression, next },
    }
}

#[cfg(test)]
mod tests {
    use capi_process::Value;

    use crate::{
        repr::{
            fragments::{FragmentPayload, Fragments},
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
                FragmentExpression::Value(Value(1)),
                FragmentExpression::Value(Value(1)),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut script = Script::default();
        script.function("f", [], |_| {});

        let mut fragments = script_to_fragments(script);

        let start = fragments.by_function.remove(0).start;
        let last_fragment = fragments.inner.drain_from(start).last().unwrap();
        assert_eq!(last_fragment.payload, FragmentPayload::Terminator);
    }

    fn body(mut fragments: Fragments) -> Vec<FragmentExpression> {
        let mut body = Vec::new();

        let start = fragments.by_function.remove(0).start;

        body.extend(fragments.inner.drain_from(start).filter_map(|fragment| {
            match fragment.payload {
                FragmentPayload::Expression { expression, .. } => {
                    Some(expression)
                }
                FragmentPayload::Terminator => None,
            }
        }));

        body
    }
}
