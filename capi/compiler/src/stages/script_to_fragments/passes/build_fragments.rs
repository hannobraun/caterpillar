use std::collections::BTreeSet;

use crate::repr::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentParent,
        FragmentPayload,
    },
    syntax::Expression,
};

use super::build_scopes::{BindingResolved, Scopes};

pub fn compile_block(
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
        let fragment = compile_expression(
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

pub fn compile_expression(
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
            let (start, environment) = compile_block(
                expressions,
                FragmentParent::Fragment { id: next },
                functions,
                scopes,
                fragments,
            );
            FragmentExpression::Block { start, environment }
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
                FragmentExpression::ResolvedUserFunction { name }
            } else if let Some(resolved) = scopes.resolve_binding(&name) {
                if let BindingResolved::InEnvironment = resolved {
                    environment.insert(name.clone());
                }
                FragmentExpression::ResolvedBinding { name }
            } else {
                // This doesn't check whether the built-in function exists, and
                // given how built-in functions are currently defined, that's
                // not practical to implement.
                FragmentExpression::ResolvedBuiltinFunction { name }
            }
        }
    };

    Fragment {
        parent,
        payload: FragmentPayload::Expression { expression, next },
    }
}
