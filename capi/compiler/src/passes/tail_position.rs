use crate::syntax::{Expression, Function};

pub fn determine_tail_positions(functions: &mut Vec<Function>) {
    for function in functions {
        analyze_function(function);
    }
}

fn analyze_function(function: &mut Function) {
    for branch in &mut function.branches {
        analyze_branch(&mut branch.body);
    }
}

fn analyze_branch(body: &mut [Expression]) {
    for expression in body.iter_mut().rev() {
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

    for expression in body {
        if let Expression::Function { function } = expression {
            analyze_function(function);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        passes::{parse, tokenize},
        syntax::{Expression, Function},
    };

    use super::determine_tail_positions;

    #[test]
    fn tail_call_in_named_function() {
        // The last expression in a function should be marked as being in tail
        // position. Others should not be.

        let mut functions = tokenize_and_parse(
            r"
                f: {
                    \ ->
                        not_tail
                        tail
                }
            ",
        );

        determine_tail_positions(&mut functions);

        let branch = functions.remove(0).branches.remove(0);
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn tail_call_in_anonymous_function() {
        // The compiler pass that determines tail positions should step into
        // nested functions.

        let mut functions = tokenize_and_parse(
            r"
                f: {
                    \ ->
                        a
                        {
                            \ ->
                                not_tail
                                tail
                        }
                        b
                }
            ",
        );

        determine_tail_positions(&mut functions);

        let mut function = functions.remove(0);
        let mut branch = function.branches.remove(0);
        let Expression::Function { function } = branch.body.remove(1) else {
            panic!("Expected block.");
        };
        let identifiers =
            function.branches.first().unwrap().body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    #[test]
    fn comment_after_tail_call() {
        // A comment being located after a tail call should not confuse the
        // analysis.

        let mut functions = tokenize_and_parse(
            r"
                f: {
                    \ ->
                        not_tail
                        tail
                        # This is a comment.
                }
            ",
        );

        determine_tail_positions(&mut functions);

        let mut function = functions.remove(0);
        let branch = function.branches.remove(0);
        let identifiers = branch.body.to_identifiers();
        assert_eq!(identifiers, vec![("not_tail", false), ("tail", true)]);
    }

    pub fn tokenize_and_parse(source: &str) -> Vec<Function> {
        let tokens = tokenize(source);
        parse(tokens)
    }

    trait ToIdentifiers {
        fn to_identifiers(&self) -> Vec<(&str, bool)>;
    }

    impl ToIdentifiers for Vec<Expression> {
        fn to_identifiers(&self) -> Vec<(&str, bool)> {
            self.iter()
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
