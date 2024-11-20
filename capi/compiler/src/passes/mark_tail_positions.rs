use crate::code::{Expression, Functions};

pub fn mark_tail_positions(functions: &mut Functions) {
    for mut function in functions.all_functions_mut() {
        for branch in function.branches.values_mut() {
            for expression in branch.body.values_mut().rev() {
                if let Expression::Comment { .. } = expression {
                    continue;
                }

                if let Expression::UnresolvedIdentifier {
                    is_known_to_be_in_tail_position,
                    ..
                } = expression
                {
                    *is_known_to_be_in_tail_position = true;
                }

                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code::{tokens::Tokens, Expression, Functions, IndexMap},
        passes::parse,
    };

    #[test]
    fn tail_call_in_named_function() {
        // The last expression in a function should be marked as being in tail
        // position. Others should not be.

        let functions = determine_tail_positions(
            r"
                f: fn
                    \ ->
                        not_tail
                        tail
                end
            ",
        );

        let (_, branch) = functions
            .named
            .into_iter()
            .next()
            .unwrap()
            .inner
            .branches
            .pop_first()
            .unwrap();
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn tail_call_in_anonymous_function() {
        // The compiler pass that determines tail positions should step into
        // nested functions.

        let functions = determine_tail_positions(
            r"
                f: fn
                    \ ->
                        a
                        fn
                            \ ->
                                not_tail
                                tail
                        end
                        b
                end
            ",
        );

        assert_eq!(
            functions
                .named
                .by_name("f")
                .unwrap()
                .into_located_function()
                .find_single_branch()
                .unwrap()
                .body()
                .nth(1)
                .and_then(|expression| {
                    functions.anonymous.by_location(&expression.location)
                })
                .unwrap()
                .find_single_branch()
                .unwrap()
                .body
                .to_identifiers(),
            vec![("not_tail", false), ("tail", true)],
        );
    }

    #[test]
    fn comment_after_tail_call() {
        // A comment being located after a tail call should not confuse the
        // analysis.

        let functions = determine_tail_positions(
            r"
                f: fn
                    \ ->
                        not_tail
                        tail
                        # This is a comment.
                end
            ",
        );

        let mut function = functions.named.into_iter().next().unwrap();
        let (_, branch) = function.inner.branches.pop_first().unwrap();
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    pub fn determine_tail_positions(input: &str) -> Functions {
        let tokens = Tokens::from_input(input);
        let mut functions = parse(tokens);
        super::mark_tail_positions(&mut functions);

        functions
    }

    trait ToIdentifiers {
        fn to_identifiers(&self) -> Vec<(&str, bool)>;
    }

    impl ToIdentifiers for IndexMap<Expression> {
        fn to_identifiers(&self) -> Vec<(&str, bool)> {
            self.values()
                .filter_map(|expression| {
                    if let Expression::UnresolvedIdentifier {
                        name,
                        is_known_to_be_in_tail_position,
                        ..
                    } = expression
                    {
                        Some((name.as_str(), *is_known_to_be_in_tail_position))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    }
}
