use std::collections::BTreeSet;

use crate::repr::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentParent,
        FragmentPayload,
    },
    syntax::Expression,
};

use super::build_scopes::Bindings;

pub fn compile_block(
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

pub fn compile_expression(
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
