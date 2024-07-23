use std::collections::{BTreeMap, BTreeSet};

use capi_process::{builtin, Host};

use crate::repr::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentParent,
        FragmentPayload, Fragments, Function,
    },
    syntax::{Expression, Script},
};

pub fn generate_fragments<H: Host>(script: Script) -> Fragments {
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
        let (start, environment) = compile_block::<H>(
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
            "Functions have no environment that they could access.\n\
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

pub fn process_function(args: Vec<String>, body: &[Expression]) -> Scopes {
    let mut scopes = Scopes {
        stack: vec![Bindings {
            inner: args.into_iter().collect(),
        }],
    };

    process_block(body, &mut scopes);

    scopes
}

fn process_block(body: &[Expression], scopes: &mut Scopes) {
    for expression in body {
        if let Expression::Binding { names } = expression {
            for name in names.iter().cloned().rev() {
                // Inserting bindings unconditionally like this does mean
                // that bindings can overwrite previously defined bindings.
                // This is undesirable, but it'll do for now.
                scopes.stack.last_mut().unwrap().inner.insert(name);
            }
        }
        if let Expression::Block { expressions } = expression {
            scopes.stack.push(Bindings {
                inner: BTreeSet::new(),
            });
            process_block(expressions, scopes);
        }
    }
}

#[derive(Debug)]
pub struct Scopes {
    stack: Vec<Bindings>,
}

impl Scopes {
    pub fn resolve_binding(&self, name: &str) -> Option<BindingResolved> {
        let mut scopes = self.stack.iter().rev();

        if let Some(scope) = scopes.next() {
            if scope.inner.contains(name) {
                return Some(BindingResolved::InScope);
            }
        }

        for scope in scopes {
            if scope.inner.contains(name) {
                return Some(BindingResolved::InEnvironment);
            }
        }

        None
    }
}

pub enum BindingResolved {
    InScope,
    InEnvironment,
}

#[derive(Debug)]
struct Bindings {
    inner: BTreeSet<String>,
}

pub fn compile_block<H: Host>(
    expressions: Vec<Expression>,
    parent: FragmentParent,
    functions: &BTreeSet<String>,
    scopes: &mut Scopes,
    fragments: &mut FragmentMap,
) -> (FragmentId, BTreeSet<String>) {
    let mut next = {
        let terminator = Fragment {
            parent: parent.clone(),
            payload: FragmentPayload::Terminator,
        };
        let terminator_id = terminator.id();

        fragments.inner.insert(terminator_id, terminator);

        terminator_id
    };
    let mut environment = BTreeSet::new();

    for expression in expressions.into_iter().rev() {
        let fragment = compile_expression::<H>(
            expression,
            parent.clone(),
            next,
            functions,
            &mut environment,
            scopes,
            fragments,
        );

        next = fragment.id();

        fragments.inner.insert(fragment.id(), fragment);
    }

    (next, environment)
}

pub fn compile_expression<H: Host>(
    expression: Expression,
    parent: FragmentParent,
    next: FragmentId,
    functions: &BTreeSet<String>,
    environment: &mut BTreeSet<String>,
    scopes: &mut Scopes,
    fragments: &mut FragmentMap,
) -> Fragment {
    let expression = match expression {
        Expression::Binding { names } => {
            FragmentExpression::BindingDefinitions { names }
        }
        Expression::Block { expressions } => {
            let (start, environment) = compile_block::<H>(
                expressions,
                FragmentParent::Fragment { id: next },
                functions,
                scopes,
                fragments,
            );
            FragmentExpression::Block { start, environment }
        }
        Expression::Comment { text } => FragmentExpression::Comment { text },
        Expression::Reference { name, .. } => {
            // The way this is written, the different types of definitions
            // shadow each other in a defined order.
            //
            // This isn't desirable. There should at least be a warning, if such
            // shadowing isn't forbidden outright. It'll do for now though.
            if functions.contains(&name) {
                FragmentExpression::ResolvedUserFunction { name }
            } else if let Some(resolved) = scopes.resolve_binding(&name) {
                if let BindingResolved::InEnvironment = resolved {
                    environment.insert(name.clone());
                }
                FragmentExpression::ResolvedBinding { name }
            } else if H::function(&name).is_some() {
                FragmentExpression::ResolvedHostFunction { name }
            } else if builtin(&name).is_some()
                || name == "return_if_non_zero"
                || name == "return_if_zero"
            {
                FragmentExpression::ResolvedBuiltinFunction { name }
            } else {
                FragmentExpression::UnresolvedWord { name }
            }
        }
        Expression::Value(value) => FragmentExpression::Value(value),
    };

    Fragment {
        parent,
        payload: FragmentPayload::Expression { expression, next },
    }
}

#[cfg(test)]
mod tests {
    use capi_process::{Effect, Host, HostFunction, Stack, Value};

    use crate::repr::{
        fragments::{
            Fragment, FragmentExpression, FragmentParent, FragmentPayload,
            Fragments,
        },
        syntax::Script,
    };

    #[test]
    fn arg_eval() {
        let mut script = Script::default();
        script.function("f", ["a"], |s| {
            s.r("a");
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
            s.v(0).bind(["b"]).r("b");
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
            s.r("brk");
        });

        let fragments = script_to_fragments(script);

        let body = body(fragments);
        assert_eq!(
            body,
            [FragmentExpression::ResolvedBuiltinFunction {
                name: String::from("brk")
            }]
        );
    }

    #[test]
    fn host_call() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.r("host");
        });

        let fragments = script_to_fragments(script);

        let body = body(fragments);
        assert_eq!(
            body,
            [FragmentExpression::ResolvedHostFunction {
                name: String::from("host")
            }]
        );
    }

    #[test]
    fn function_call() {
        let mut script = Script::default();
        script.function("f", [], |s| {
            s.r("g");
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

    fn script_to_fragments(script: Script) -> Fragments {
        super::generate_fragments::<TestHost>(script)
    }

    struct TestHost {}

    impl Host for TestHost {
        type Effect = ();

        fn function(name: &str) -> Option<HostFunction<Self::Effect>> {
            match name {
                "host" => Some(host),
                _ => None,
            }
        }
    }

    fn host(_: &mut Stack) -> Result<(), Effect<()>> {
        Ok(())
    }
}
