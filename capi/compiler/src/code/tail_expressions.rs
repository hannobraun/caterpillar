use std::collections::BTreeSet;

use super::{Expression, ExpressionLocation, Functions};

/// # Tracks tail expressions
///
/// A tail expression is the last expression in a function. Any function that
/// has at least one expression, has a tail expression.
///
/// Knowing if a given expression is a tail, is relevant if that expression is a
/// call to a user-defined function. Those tail calls get eliminated (which is
/// called "tail call elimination", or "tail call optimization"), to reduce
/// stack usage and prevent unlimited stack growth in the presence of recursive
/// tail calls.
#[derive(Debug)]
pub struct TailExpressions {
    inner: BTreeSet<ExpressionLocation>,
}

impl TailExpressions {
    /// # Find all tail expressions
    pub fn from_functions(functions: &Functions) -> Self {
        let mut tail_expressions = BTreeSet::new();

        for function in functions.all_functions() {
            for branch in function.branches() {
                for expression in branch.body().rev() {
                    if let Expression::Comment { .. } = expression.fragment {
                        continue;
                    }

                    if let Expression::UnresolvedIdentifier { .. } =
                        expression.fragment
                    {
                        tail_expressions.insert(expression.location);
                    }

                    break;
                }
            }
        }

        Self {
            inner: tail_expressions,
        }
    }

    /// # Determine if the expression at the provided location is a tail
    pub fn is_tail_expression(&self, location: &ExpressionLocation) -> bool {
        self.inner.contains(location)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::code::{syntax::parse, tokens::Tokens, Functions};

    use super::TailExpressions;

    #[test]
    fn find_tail_calls() {
        // The last expression in a function should be marked as being a tail.
        // Others should not be.

        let (functions, tail_expressions) = tail_expressions(
            r"
                f: fn
                    \ ->
                        not_tail
                        tail
                end
            ",
        );

        let (not_tail, tail) = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(!tail_expressions.is_tail_expression(&not_tail));
        assert!(tail_expressions.is_tail_expression(&tail));
    }

    #[test]
    fn ignore_comments() {
        // A comment being located after a tail call should not confuse the
        // analysis.

        let (functions, tail_expressions) = tail_expressions(
            r"
                f: fn
                    \ ->
                        not_tail
                        tail
                        # This is a comment.
                end
            ",
        );

        let tail = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .nth(1)
            .unwrap();

        assert!(tail_expressions.is_tail_expression(&tail));
    }

    pub fn tail_expressions(input: &str) -> (Functions, TailExpressions) {
        let tokens = Tokens::from_input(input);
        let functions = parse(tokens);
        let tail_expressions = TailExpressions::from_functions(&functions);

        (functions, tail_expressions)
    }
}
