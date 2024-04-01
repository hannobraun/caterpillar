use std::collections::BTreeSet;

use super::syntax::SyntaxElement;

pub fn resolve(
    syntax: Vec<SyntaxElement>,
    functions: &BTreeSet<&'static str>,
) -> Vec<Expression> {
    syntax
        .into_iter()
        .map(|syntax_element| match syntax_element {
            SyntaxElement::Value(value) => Expression::Value(value),
            SyntaxElement::Word { name } => {
                // The code here would allow user-defined functions to shadow
                // built-in functions, which seems undesirable. It's better to
                // catch this when defining the function though, and while it
                // would be nice to have a fallback assertion here, that's not
                // practical, given the way built-in function resolution is
                // implemented right now.

                if functions.contains(name) {
                    return Expression::Function { name };
                }

                Expression::Builtin { name }
            }
        })
        .collect()
}

pub enum Expression {
    Builtin { name: &'static str },
    Function { name: &'static str },
    Value(usize),
}
